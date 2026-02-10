//! Mesa's Rust SDK
//!
//! Provides both an ergonomic [`MesaClient`] and direct access to the
//! lower-level modules generated from the `OpenAPI` spec.
//!
//! The [`low_level`] module contains the generated API functions and
//! hand-written workarounds for endpoints where the code generator
//! produces incorrect types.
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use mesa_dev::MesaClient;
//! use futures::TryStreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = MesaClient::builder().build();
//!
//! // List repositories
//! let repos: Vec<_> = client.org("my-org").repos().list(None).try_collect().await?;
//!
//! // Get file content
//! let content = client
//!     .org("my-org")
//!     .repos().at("my-repo")
//!     .content()
//!     .get(None, Some("README.md"), None)
//!     .await?;
//! # Ok(())
//! # }
//! ```

pub mod client;
pub mod low_level;

pub use client::MesaClient;

/// Re-export of [`futures_core::Stream`] for consuming paginated results.
pub use futures_core::Stream;

/// OpenAPI-generated request and response model types.
///
/// All structs implement `serde::Serialize` and `serde::Deserialize`.
#[doc(inline)]
pub use mesa_dev_oapi::models;
