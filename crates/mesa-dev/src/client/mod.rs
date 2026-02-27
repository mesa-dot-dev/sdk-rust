//! Ergonomic client for the Mesa API.
//!
//! Provides a directory-style navigation pattern:
//!
//! ```rust,no_run
//! use mesa_dev::MesaClient;
//! use futures::TryStreamExt;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let client = MesaClient::builder()
//!     .build()?;
//! let repos: Vec<_> = client.org("my-org").repos().list(None).try_collect().await?;
//! let branches: Vec<_> = client.org("my-org").repos().at("my-repo").branches().list(None).try_collect().await?;
//! # Ok(())
//! # }
//! ```

mod api_keys;
mod branches;
mod change;
mod commits;
mod content;
mod org;
mod repo;
mod repos;
mod webhooks;

mod pagination;

pub use api_keys::ApiKeysClient;
pub use branches::BranchesClient;
pub use change::ChangeClient;
pub use commits::CommitsClient;
pub use content::ContentClient;
pub use org::OrgClient;
pub use repo::RepoClient;
pub use repos::ReposClient;
pub use webhooks::WebhooksClient;

use crate::low_level::apis::configuration::Configuration;

/// Default gRPC endpoint for the Mesa VCS data plane.
pub const DEFAULT_GRPC_ENDPOINT: &str = "https://vcs.depot.mesa.dev";

/// Error returned when building a [`MesaClient`] fails.
#[derive(Debug)]
pub enum BuildError {
    /// The gRPC endpoint URL is invalid.
    InvalidGrpcEndpoint(tonic::codegen::http::uri::InvalidUri),
    /// TLS configuration for the gRPC endpoint failed.
    TlsConfig(tonic::transport::Error),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidGrpcEndpoint(e) => write!(f, "invalid gRPC endpoint: {e}"),
            Self::TlsConfig(e) => write!(f, "gRPC TLS configuration failed: {e}"),
        }
    }
}

impl std::error::Error for BuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidGrpcEndpoint(e) => Some(e),
            Self::TlsConfig(e) => Some(e),
        }
    }
}

/// Builder for configuring and constructing a [`MesaClient`].
#[derive(Clone, Debug, Default)]
pub struct MesaClientBuilder {
    base_path: Option<String>,
    user_agent: Option<String>,
    client: Option<reqwest_middleware::ClientWithMiddleware>,
    api_key: Option<String>,
    grpc_endpoint: Option<String>,
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

    /// Override the gRPC endpoint URL.
    ///
    /// Defaults to [`DEFAULT_GRPC_ENDPOINT`].
    #[must_use]
    pub fn with_grpc_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.grpc_endpoint = Some(endpoint.into());
        self
    }

    /// Finalize the builder and construct a [`MesaClient`].
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] if the gRPC endpoint URL is invalid.
    pub fn build(self) -> Result<MesaClient, BuildError> {
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

        let endpoint_str = self
            .grpc_endpoint
            .unwrap_or_else(|| DEFAULT_GRPC_ENDPOINT.to_owned());
        let mut endpoint = tonic::transport::Channel::from_shared(endpoint_str)
            .map_err(BuildError::InvalidGrpcEndpoint)?
            .http2_adaptive_window(true);

        // Enable TLS when the endpoint uses HTTPS.
        if endpoint
            .uri()
            .scheme_str()
            .is_some_and(|s| s.eq_ignore_ascii_case("https"))
        {
            endpoint = endpoint
                .tls_config(
                    tonic::transport::ClientTlsConfig::new()
                        .with_native_roots()
                        .with_enabled_roots(),
                )
                .map_err(BuildError::TlsConfig)?;
        }

        let grpc_channel = endpoint.connect_lazy();

        Ok(MesaClient {
            config,
            grpc_channel,
        })
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
    pub(crate) config: Configuration,
    pub(crate) grpc_channel: tonic::transport::Channel,
}

impl MesaClient {
    /// Create a new builder with default configuration.
    #[must_use]
    pub fn builder() -> MesaClientBuilder {
        MesaClientBuilder::default()
    }

    /// Create a new client from an existing [`Configuration`] and gRPC channel.
    #[must_use]
    pub fn from_configuration(
        config: Configuration,
        grpc_channel: tonic::transport::Channel,
    ) -> Self {
        Self {
            config,
            grpc_channel,
        }
    }

    /// Navigate to an organization.
    #[must_use]
    pub fn org<'a>(&'a self, name: &'a str) -> OrgClient<'a> {
        OrgClient {
            client: self,
            org: name,
        }
    }
}
