//! Ergonomic client for the Mesa API.
//!
//! Provides a directory-style navigation pattern:
//!
//! ```rust,no_run
//! use mesa_dev::MesaClient;
//! use futures::TryStreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = MesaClient::builder().build();
//! let repos: Vec<_> = client.org("my-org").repos().list(None).try_collect().await?;
//! let branches: Vec<_> = client.org("my-org").repos().at("my-repo").branches().list(None).try_collect().await?;
//! # Ok(())
//! # }
//! ```

mod analytics;
mod api_keys;
mod branches;
mod commits;
mod content;
mod diff;
mod lfs;
mod org;
mod repo;
mod repos;
mod sync;
mod webhooks;

mod pagination;

pub use analytics::AnalyticsClient;
pub use api_keys::ApiKeysClient;
pub use branches::BranchesClient;
pub use commits::CommitsClient;
pub use content::ContentClient;
pub use diff::DiffClient;
pub use lfs::LfsClient;
pub use org::OrgClient;
pub use repo::RepoClient;
pub use repos::ReposClient;
pub use sync::SyncClient;
pub use webhooks::WebhooksClient;

use crate::low_level::apis::configuration::Configuration;

/// Builder for configuring and constructing a [`MesaClient`].
#[derive(Clone, Debug, Default)]
pub struct MesaClientBuilder {
    base_path: Option<String>,
    user_agent: Option<String>,
    client: Option<reqwest_middleware::ClientWithMiddleware>,
    api_key: Option<String>,
}

impl MesaClientBuilder {
    /// Attach a non-default base URL for the API (e.g. for testing against a staging environment).
    #[must_use]
    pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
        self.base_path = Some(base_path.into());
        self
    }

    /// Attach a custom User-Agent header to all requests.
    #[must_use]
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Attach a custom HTTP client (e.g. with additional middleware or custom timeout settings).
    #[must_use]
    pub fn with_client(mut self, client: reqwest_middleware::ClientWithMiddleware) -> Self {
        self.client = Some(client);
        self
    }

    /// Attach an API key for authentication.
    #[must_use]
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Finalize the builder and construct a [`MesaClient`].
    #[must_use]
    pub fn build(self) -> MesaClient {
        let mut config = Configuration::default();

        if let Some(base_path) = self.base_path {
            config.base_path = base_path;
        }

        config.user_agent = self.user_agent.clone().or(Some(Self::default_user_agent()));
        if let Some(client) = self.client {
            config.client = client;
        }

        if let Some(api_key) = self.api_key {
            config.bearer_access_token = Some(api_key);
        }

        MesaClient { config }
    }

    fn default_user_agent() -> String {
        format!(
            "mesa-dev/{} (rust/{})",
            env!("CARGO_PKG_VERSION"),
            env!("MESA_RUSTC_VERSION"),
        )
    }
}

/// Top-level Mesa API client.
///
/// Create one with [`MesaClient::builder`] or [`MesaClient::from_configuration`]
/// and navigate to sub-resources with [`MesaClient::org`].
#[derive(Clone, Debug)]
pub struct MesaClient {
    config: Configuration,
}

impl MesaClient {
    /// Create a new builder with default configuration.
    #[must_use]
    pub fn builder() -> MesaClientBuilder {
        MesaClientBuilder::default()
    }

    /// Create a new client from an existing [`Configuration`].
    #[must_use]
    pub fn from_configuration(config: Configuration) -> Self {
        Self { config }
    }

    /// Navigate to an organization.
    #[must_use]
    pub fn org<'a>(&'a self, name: &'a str) -> OrgClient<'a> {
        OrgClient {
            config: &self.config,
            org: name,
        }
    }
}
