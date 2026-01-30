//! HTTP client abstraction for pluggable backends.
//!
//! The [`HttpClient`] trait is the extension point for bringing your own HTTP
//! transport. The SDK ships with two optional implementations:
//!
//! - [`ReqwestClient`](crate::backends::ReqwestClient) — async, via [reqwest](https://docs.rs/reqwest) (default)
//! - [`UreqClient`](crate::backends::UreqClient) — blocking, via [ureq](https://docs.rs/ureq)
//!
//! To use a different HTTP library, implement [`HttpClient`] and pass your
//! implementation to [`ClientBuilder::build_with`](crate::ClientBuilder::build_with).

use bytes::Bytes;
use http::{HeaderMap, Method, StatusCode};
use std::future::Future;

use crate::error::HttpClientError;

/// An HTTP request to be sent by an [`HttpClient`] implementation.
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// The HTTP method.
    pub method: Method,
    /// The fully-qualified URL.
    pub url: String,
    /// Request headers.
    pub headers: HeaderMap,
    /// Optional request body.
    pub body: Option<Bytes>,
}

/// An HTTP response returned by an [`HttpClient`] implementation.
#[derive(Debug)]
pub struct HttpResponse {
    /// The HTTP status code.
    pub status: StatusCode,
    /// Response headers.
    pub headers: HeaderMap,
    /// Response body bytes.
    pub body: Bytes,
}

/// Trait for pluggable HTTP client backends.
///
/// Implement this trait to use any HTTP library with the Mesa SDK. The SDK
/// handles authorization headers, JSON serialization, and retry logic — your
/// implementation only needs to execute the raw HTTP request and return the
/// response.
///
/// # What the SDK handles
///
/// Before calling [`send`](HttpClient::send), the SDK:
/// - Sets the `Authorization: Bearer <api-key>` header
/// - Sets `Content-Type: application/json` for requests with a body
/// - Sets `Accept: application/json`
/// - Serializes request bodies to JSON bytes
///
/// After [`send`](HttpClient::send) returns, the SDK:
/// - Deserializes the response body from JSON
/// - Checks the status code for errors
/// - Retries on transient failures (429, 5xx, timeouts, connection errors)
///
/// # What you implement
///
/// Your [`send`](HttpClient::send) method should:
/// 1. Execute the HTTP request described by [`HttpRequest`]
/// 2. Return the status code, headers, and raw body bytes in [`HttpResponse`]
/// 3. Map transport errors to [`HttpClientError`] variants
///
/// # Error mapping for retries
///
/// The SDK uses [`HttpClientError`] variants to decide whether to retry:
/// - [`Timeout`](HttpClientError::Timeout) → retried
/// - [`Connection`](HttpClientError::Connection) → retried
/// - [`Other`](HttpClientError::Other) → **not** retried
///
/// Map your HTTP library's errors accordingly to get correct retry behavior.
///
/// # Example
///
/// ```rust
/// use mesa_dev::{ClientBuilder, HttpClient, HttpRequest, HttpResponse, error::HttpClientError};
///
/// struct MyClient;
///
/// impl HttpClient for MyClient {
///     async fn send(
///         &self,
///         request: HttpRequest,
///     ) -> Result<HttpResponse, HttpClientError> {
///         // Execute request.method against request.url with request.headers
///         // and optional request.body, then return HttpResponse.
///         todo!()
///     }
/// }
///
/// let client = ClientBuilder::new("my-api-key").build_with(MyClient);
/// ```
pub trait HttpClient: Send + Sync {
    /// Send an HTTP request and return the response.
    fn send(
        &self,
        request: HttpRequest,
    ) -> impl Future<Output = Result<HttpResponse, HttpClientError>> + Send;
}
