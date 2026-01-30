//! Async HTTP backend powered by [reqwest](https://docs.rs/reqwest).
//!
//! This is the default backend, enabled by the `reqwest-client` Cargo feature.
//! It requires an async runtime (typically Tokio).

use std::time::Duration;

use crate::error::HttpClientError;
use crate::http_client::{HttpClient, HttpRequest, HttpResponse};

/// Async HTTP backend powered by [reqwest](https://docs.rs/reqwest).
///
/// This is the default backend, enabled by the `reqwest-client` feature flag.
/// It requires an async runtime (typically Tokio).
///
/// # Construction
///
/// Usually you don't need to construct this directly — [`Mesa::new`](crate::Mesa::new)
/// and [`ClientBuilder::build`](crate::ClientBuilder::build) create it for you.
///
/// If you need a custom `reqwest::Client` (e.g., with a proxy or custom TLS):
///
/// ```rust,no_run
/// use mesa_dev::{ClientBuilder, ReqwestClient};
///
/// let reqwest_client = reqwest::Client::builder()
///     .danger_accept_invalid_certs(true) // for development only!
///     .build()
///     .unwrap();
///
/// let client = ClientBuilder::new("my-api-key")
///     .build_with(ReqwestClient::from_client(reqwest_client));
/// ```
#[derive(Debug, Clone)]
pub struct ReqwestClient {
    client: reqwest::Client,
}

impl ReqwestClient {
    /// Create a new `ReqwestClient` with the given timeout.
    #[must_use]
    pub fn new(timeout: Duration) -> Self {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .unwrap_or_default();
        Self { client }
    }

    /// Create a `ReqwestClient` from an existing [`reqwest::Client`].
    #[must_use]
    pub fn from_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl HttpClient for ReqwestClient {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, HttpClientError> {
        let mut builder = self.client.request(request.method, &request.url);
        builder = builder.headers(request.headers);

        if let Some(body) = request.body {
            builder = builder.body(body);
        }

        let response = builder.send().await.map_err(map_reqwest_error)?;

        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await.map_err(map_reqwest_error)?;

        Ok(HttpResponse {
            status,
            headers,
            body,
        })
    }
}

/// Map a reqwest error to our [`HttpClientError`].
fn map_reqwest_error(err: reqwest::Error) -> HttpClientError {
    if err.is_timeout() {
        HttpClientError::Timeout
    } else if err.is_connect() {
        HttpClientError::Connection(err.to_string())
    } else {
        HttpClientError::Other(Box::new(err))
    }
}
