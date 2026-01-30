//! Cursor-based pagination support.
//!
//! Paginated API endpoints return results in pages. This module provides
//! [`PageStream`], a lazy async iterator that fetches pages on demand.
//!
//! # Usage
//!
//! Most users should use the `list_all()` methods on resource types, which
//! return a `PageStream`:
//!
//! ```rust,no_run
//! # use mesa_dev::{Mesa, MesaError};
//! # async fn run() -> Result<(), MesaError> {
//! # let client = Mesa::new("key");
//! // Collect all items
//! let repos = client.repos("org").list_all().collect().await?;
//!
//! // Or iterate one at a time
//! let mut stream = client.repos("org").list_all();
//! while let Some(repo) = stream.next().await? {
//!     println!("{}", repo.name);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # As a `futures::Stream`
//!
//! `PageStream` implements [`futures_core::Stream`], so you can use it with
//! any stream combinator:
//!
//! ```rust,no_run
//! # use mesa_dev::{Mesa, MesaError};
//! use futures::{pin_mut, StreamExt};
//!
//! # async fn run() -> Result<(), MesaError> {
//! # let client = Mesa::new("key");
//! let stream = client.repos("org").list_all();
//! pin_mut!(stream);
//!
//! while let Some(result) = stream.next().await {
//!     let repo = result?;
//!     println!("{}", repo.name);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! For manual page-by-page control, use the `list()` methods with
//! [`PaginationParams`](crate::models::PaginationParams).

use std::collections::VecDeque;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use http::Method;
use serde::de::DeserializeOwned;

use crate::client::ClientInner;
use crate::error::MesaError;
use crate::http_client::HttpClient;
use crate::models::Paginated;

/// A boxed, Send future that resolves to an optional page or error.
type FetchFuture<Page> = Pin<Box<dyn Future<Output = Result<Option<Page>, MesaError>> + Send>>;

/// An async page stream that lazily fetches pages from a paginated endpoint.
///
/// Created by `list_all()` methods on resource types. Owns all its state via
/// `Arc`, so it has no lifetime parameters and can be stored freely.
///
/// # Methods
///
/// - [`next()`](Self::next) — Get the next individual item, fetching new pages
///   as needed. Returns `Ok(None)` when exhausted.
/// - [`next_page()`](Self::next_page) — Get the next full page of results.
///   Returns `Ok(None)` when exhausted.
/// - [`collect()`](Self::collect) — Consume the stream and collect all
///   remaining items into a `Vec`.
///
/// # `futures::Stream`
///
/// This type also implements [`futures_core::Stream`] with
/// `Item = Result<Page::Item, MesaError>`, enabling use with `StreamExt`
/// combinators like `.map()`, `.filter()`, `.take()`, etc.
pub struct PageStream<C: HttpClient, Page: Paginated + DeserializeOwned> {
    inner: Arc<ClientInner<C>>,
    path: String,
    extra_query: Vec<(String, String)>,
    cursor: Option<String>,
    buffer: VecDeque<Page::Item>,
    done: bool,
    fetch_future: Option<FetchFuture<Page>>,
}

// All fields are Unpin (Arc, String, Vec, Option, VecDeque, bool, and
// Pin<Box<..>> which is Unpin because Box is Unpin). This lets us access
// &mut self freely inside poll_next without pin projection.
impl<C: HttpClient, Page: Paginated + DeserializeOwned> Unpin for PageStream<C, Page> {}

impl<C: HttpClient, Page: Paginated + DeserializeOwned> PageStream<C, Page> {
    /// Create a new page stream.
    pub(crate) fn new(
        inner: Arc<ClientInner<C>>,
        path: String,
        extra_query: Vec<(String, String)>,
    ) -> Self {
        Self {
            inner,
            path,
            extra_query,
            cursor: None,
            buffer: VecDeque::new(),
            done: false,
            fetch_future: None,
        }
    }

    /// Fetch the next individual item, requesting new pages as needed.
    ///
    /// Returns `Ok(None)` when all pages have been exhausted.
    ///
    /// # Errors
    ///
    /// Returns [`MesaError`] if the API request fails.
    pub async fn next(&mut self) -> Result<Option<Page::Item>, MesaError> {
        if let Some(item) = self.buffer.pop_front() {
            return Ok(Some(item));
        }

        if self.done {
            return Ok(None);
        }

        if let Some(page) = self.fetch_page().await? {
            let has_more = page.has_more();
            self.cursor = page.next_cursor().map(ToOwned::to_owned);
            self.buffer = VecDeque::from(page.items());
            self.done = !has_more || self.cursor.is_none();
            Ok(self.buffer.pop_front())
        } else {
            self.done = true;
            Ok(None)
        }
    }

    /// Collect all remaining items into a `Vec`.
    ///
    /// # Errors
    ///
    /// Returns [`MesaError`] if any API request fails.
    pub async fn collect(mut self) -> Result<Vec<Page::Item>, MesaError> {
        let mut all = Vec::new();
        while let Some(item) = self.next().await? {
            all.push(item);
        }
        Ok(all)
    }

    /// Fetch the next full page.
    ///
    /// Returns `Ok(None)` when all pages have been exhausted.
    ///
    /// # Errors
    ///
    /// Returns [`MesaError`] if the API request fails.
    pub async fn next_page(&mut self) -> Result<Option<Page>, MesaError> {
        if self.done {
            return Ok(None);
        }
        let page = self.fetch_page().await?;
        if let Some(ref p) = page {
            let has_more = p.has_more();
            self.cursor = p.next_cursor().map(ToOwned::to_owned);
            self.done = !has_more || self.cursor.is_none();
        } else {
            self.done = true;
        }
        Ok(page)
    }

    /// Internal: fetch a single page from the API.
    async fn fetch_page(&self) -> Result<Option<Page>, MesaError> {
        let mut query: Vec<(&str, &str)> = self
            .extra_query
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();

        if let Some(ref cursor) = self.cursor {
            query.push(("cursor", cursor));
        }

        let page: Page = self
            .inner
            .request(Method::GET, &self.path, &query, None)
            .await?;

        Ok(Some(page))
    }
}

/// Fetch a single page from the API with all owned arguments.
///
/// This standalone function captures owned data so the returned future is
/// `'static`, which is required for the `Stream` implementation.
async fn fetch_page_owned<C: HttpClient, Page: Paginated + DeserializeOwned>(
    inner: Arc<ClientInner<C>>,
    path: String,
    extra_query: Vec<(String, String)>,
    cursor: Option<String>,
) -> Result<Option<Page>, MesaError> {
    let mut query: Vec<(&str, &str)> = extra_query
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect();

    if let Some(ref c) = cursor {
        query.push(("cursor", c));
    }

    let page: Page = inner.request(Method::GET, &path, &query, None).await?;
    Ok(Some(page))
}

impl<C, Page> futures_core::Stream for PageStream<C, Page>
where
    C: HttpClient + 'static,
    Page: Paginated + DeserializeOwned + 'static,
{
    type Item = Result<Page::Item, MesaError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();

        // Return buffered items first.
        if let Some(item) = this.buffer.pop_front() {
            return Poll::Ready(Some(Ok(item)));
        }

        // If done, signal end of stream.
        if this.done {
            return Poll::Ready(None);
        }

        // If no fetch is in flight, start one.
        if this.fetch_future.is_none() {
            this.fetch_future = Some(Box::pin(fetch_page_owned::<C, Page>(
                Arc::clone(&this.inner),
                this.path.clone(),
                this.extra_query.clone(),
                this.cursor.clone(),
            )));
        }

        // Poll the in-flight fetch.
        let Some(fut) = this.fetch_future.as_mut() else {
            this.done = true;
            return Poll::Ready(None);
        };

        match fut.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => {
                this.fetch_future = None;

                match result {
                    Ok(Some(page)) => {
                        let has_more = page.has_more();
                        this.cursor = page.next_cursor().map(ToOwned::to_owned);
                        this.buffer = VecDeque::from(page.items());
                        this.done = !has_more || this.cursor.is_none();
                        if let Some(item) = this.buffer.pop_front() {
                            Poll::Ready(Some(Ok(item)))
                        } else {
                            this.done = true;
                            Poll::Ready(None)
                        }
                    }
                    Ok(None) => {
                        this.done = true;
                        Poll::Ready(None)
                    }
                    Err(e) => {
                        this.done = true;
                        Poll::Ready(Some(Err(e)))
                    }
                }
            }
        }
    }
}
