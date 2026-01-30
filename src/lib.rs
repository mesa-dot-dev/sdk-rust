//! # mesa-dev
//!
//! Rust SDK for the [mesa.dev](https://mesa.dev) API.
//!
//! **Note:** A large part of this documentation was generated with the assistance
//! of an LLM. If you spot inaccuracies, please open an issue.
//!
//! This crate provides a typed, async-first client for managing repositories,
//! branches, commits, content, diffs, and API keys on the Mesa platform.
//!
//! # Quick Start
//!
//! With the default `reqwest-client` feature enabled:
//!
//! ```rust,no_run
//! use mesa_dev::{Mesa, MesaError, models::CreateRepoRequest};
//!
//! # async fn run() -> Result<(), MesaError> {
//! let client = Mesa::new("my-api-key");
//!
//! let repo = client
//!     .repos("my-org")
//!     .create(&CreateRepoRequest {
//!         name: "my-repo".to_owned(),
//!         default_branch: None,
//!     })
//!     .await?;
//!
//! // Auto-paginate through all branches
//! let branches = client
//!     .branches("my-org", "my-repo")
//!     .list_all()
//!     .collect()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Client Configuration
//!
//! Use [`ClientBuilder`] for fine-grained control over timeouts, retries, and
//! base URL:
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use mesa_dev::ClientBuilder;
//!
//! let client = ClientBuilder::new("my-api-key")
//!     .timeout(Duration::from_secs(60))
//!     .max_retries(5)
//!     .build();
//! ```
//!
//! # Custom HTTP Backends
//!
//! The SDK is generic over its HTTP transport. Implement [`HttpClient`] to use
//! any HTTP library, add middleware, or mock requests in tests:
//!
//! ```rust
//! use mesa_dev::{ClientBuilder, HttpClient, HttpRequest, HttpResponse, error::HttpClientError};
//!
//! struct MyClient;
//!
//! impl HttpClient for MyClient {
//!     async fn send(
//!         &self,
//!         request: HttpRequest,
//!     ) -> Result<HttpResponse, HttpClientError> {
//!         todo!("implement your HTTP transport here")
//!     }
//! }
//!
//! let client = ClientBuilder::new("my-api-key").build_with(MyClient);
//! ```
//!
//! See the [`HttpClient`] trait documentation for a detailed guide on
//! implementing custom backends, including how to map errors for correct retry
//! behavior.
//!
//! # Feature Flags
//!
//! | Feature | Description | Default |
//! |---------|-------------|---------|
//! | `reqwest-client` | Async HTTP backend via reqwest | Yes |
//! | `ureq-client` | Blocking HTTP backend via ureq | No |
//!
//! To disable all built-in backends (for custom implementations only), use:
//!
//! ```toml
//! mesa-dev = { version = "0.1", default-features = false }
//! ```

pub mod backends;
mod client;
pub mod error;
mod http_client;
pub mod models;
mod pagination;
pub mod resources;

#[cfg(feature = "reqwest-client")]
pub use client::Mesa;
pub use client::{ClientBuilder, ClientConfig, MesaClient};

pub use error::{ApiErrorCode, HttpClientError, MesaError};

pub use http_client::{HttpClient, HttpRequest, HttpResponse};

pub use pagination::PageStream;

#[cfg(feature = "reqwest-client")]
pub use backends::ReqwestClient;
#[cfg(feature = "ureq-client")]
pub use backends::UreqClient;
