//! Pagination types.
//!
//! This module provides [`PaginationParams`] for manual page-by-page control
//! and the [`Paginated`] trait that list response types implement for automatic
//! pagination via [`PageStream`](crate::PageStream).

use serde::Serialize;

/// Query parameters for paginated endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PaginationParams {
    /// Cursor for the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Maximum number of results per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

/// Trait for paginated API response types.
///
/// Implement this trait on your response type to enable automatic pagination
/// via [`PageStream`](crate::PageStream). The SDK provides implementations for
/// all built-in list response types.
///
/// # Required methods
///
/// - [`items`](Self::items) — Extract the items from this page (consumes `self`)
/// - [`next_cursor`](Self::next_cursor) — Return the cursor string for fetching the next page
/// - [`has_more`](Self::has_more) — Return whether more pages exist
pub trait Paginated {
    /// The individual item type within a page.
    type Item: Send + Sync;

    /// Returns the items from this page.
    fn items(self) -> Vec<Self::Item>;

    /// Returns the cursor for the next page, if any.
    fn next_cursor(&self) -> Option<&str>;

    /// Returns whether more pages are available.
    fn has_more(&self) -> bool;
}
