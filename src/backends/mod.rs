//! Built-in HTTP client backend implementations.
//!
//! The SDK ships with two optional backends, controlled by Cargo features:
//!
//! - **`reqwest-client`** (default) — [`ReqwestClient`], an async backend
//!   powered by [reqwest](https://docs.rs/reqwest). Use this with Tokio or any
//!   async runtime.
//!
//! - **`ureq-client`** — [`UreqClient`], a blocking backend powered by
//!   [ureq](https://docs.rs/ureq). Suitable for scripts, CLIs, and
//!   environments without an async runtime.
//!
//! To use a completely custom HTTP transport, implement [`HttpClient`](crate::HttpClient)
//! and pass it to [`ClientBuilder::build_with`](crate::ClientBuilder::build_with).

#[cfg(feature = "reqwest-client")]
mod reqwest_client;
#[cfg(feature = "reqwest-client")]
pub use reqwest_client::ReqwestClient;

#[cfg(feature = "ureq-client")]
mod ureq_client;
#[cfg(feature = "ureq-client")]
pub use ureq_client::UreqClient;
