use mesa_dev::low_level::apis::configuration::Configuration;
use mesa_dev::low_level::apis::{admin_api, repos_api};
use mesa_dev::low_level::commits::{self, CommitAuthor, CommitFile, CommitResponse};
use mesa_dev::models;
use mesa_dev::MesaClient;
use std::env;
use test_context::AsyncTestContext;
use uuid::Uuid;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Build a `Configuration` from environment variables.
///
/// Set `MESA_TEST_PROXY` (e.g. `http://localhost:8888`) to route all requests
/// through a debugging proxy such as Charles.
pub fn test_config() -> Configuration {
    let base_url = env::var("MESA_TEST_BASE_URL")
        .unwrap_or_else(|_| "https://depot.mesa.dev/api/v1".to_string());
    let api_key = env::var("MESA_TEST_API_KEY").expect("MESA_TEST_API_KEY must be set");

    let mut builder = reqwest::Client::builder();
    if let Ok(proxy_url) = env::var("MESA_TEST_PROXY") {
        builder = builder
            .proxy(reqwest::Proxy::all(&proxy_url).expect("invalid MESA_TEST_PROXY URL"))
            .danger_accept_invalid_certs(true);
    }
    let client = reqwest_middleware::ClientBuilder::new(builder.build().unwrap()).build();

    Configuration {
        base_path: base_url,
        bearer_access_token: Some(api_key),
        client,
        ..Configuration::default()
    }
}

/// Return the test organisation name.
pub fn test_org() -> String {
    env::var("MESA_TEST_ORG").expect("MESA_TEST_ORG must be set")
}

/// Generate a unique name like `test-{prefix}-{8-hex-chars}`.
pub fn unique_name(prefix: &str) -> String {
    let id = &Uuid::new_v4().to_string()[..8];
    format!("test-{prefix}-{id}")
}

// ---------------------------------------------------------------------------
// Commit helper — thin wrapper around `mesa_dev::low_level::commits::create_commit`
// ---------------------------------------------------------------------------

/// Create a commit with upsert file operations (convenience for tests).
pub async fn create_commit(
    config: &Configuration,
    org: &str,
    repo: &str,
    branch: &str,
    message: &str,
    files: &[(&str, &str)],
) -> CommitResponse {
    let author = CommitAuthor::new("Test Author".to_string(), "test@test.com".to_string());
    let commit_files: Vec<CommitFile> = files
        .iter()
        .map(|(path, content)| CommitFile::Upsert {
            path: (*path).to_string(),
            content: (*content).to_string(),
            encoding: None,
        })
        .collect();

    commits::create_commit(
        config,
        org,
        repo,
        branch,
        message,
        &author,
        &commit_files,
        None,
    )
    .await
    .expect("create_commit failed")
}

// ---------------------------------------------------------------------------
// Contexts
// ---------------------------------------------------------------------------

/// An empty repository, deleted on teardown.
pub struct RepoContext {
    pub config: Configuration,
    pub org: String,
    pub repo_name: String,
}

impl AsyncTestContext for RepoContext {
    async fn setup() -> Self {
        let config = test_config();
        let org = test_org();
        let repo_name = unique_name("repo");

        let req = models::PostByOrgReposRequest::new(repo_name.clone());
        repos_api::post_by_org_repos(&config, &org, Some(req))
            .await
            .expect("failed to create test repo");

        Self {
            config,
            org,
            repo_name,
        }
    }

    async fn teardown(self) {
        let _ = repos_api::delete_by_org_by_repo(&self.config, &self.org, &self.repo_name).await;
    }
}

/// A repository with one initial commit (README.md), deleted on teardown.
pub struct RepoWithCommitContext {
    pub config: Configuration,
    pub org: String,
    pub repo_name: String,
    pub commit_sha: String,
}

impl AsyncTestContext for RepoWithCommitContext {
    async fn setup() -> Self {
        let config = test_config();
        let org = test_org();
        let repo_name = unique_name("repoc");

        let req = models::PostByOrgReposRequest::new(repo_name.clone());
        repos_api::post_by_org_repos(&config, &org, Some(req))
            .await
            .expect("failed to create test repo");

        let commit = create_commit(
            &config,
            &org,
            &repo_name,
            "main",
            "Initial commit",
            &[("README.md", "# Test Repository")],
        )
        .await;

        Self {
            config,
            org,
            repo_name,
            commit_sha: commit.sha,
        }
    }

    async fn teardown(self) {
        let _ = repos_api::delete_by_org_by_repo(&self.config, &self.org, &self.repo_name).await;
    }
}

/// An API key, revoked on teardown.
pub struct ApiKeyContext {
    pub config: Configuration,
    pub org: String,
    pub key_id: String,
}

impl AsyncTestContext for ApiKeyContext {
    async fn setup() -> Self {
        let config = test_config();
        let org = test_org();

        let req = models::PostByOrgApiKeysRequest {
            name: Some(Some("integration-test-key".to_string())),
            scopes: Some(vec![
                models::post_by_org_api_keys_request::Scopes::GitColonRead,
                models::post_by_org_api_keys_request::Scopes::RepoColonRead,
            ]),
        };

        let resp = admin_api::post_by_org_api_keys(&config, &org, Some(req))
            .await
            .expect("failed to create test api key");

        let key_id = resp.id.expect("api key id missing");

        Self {
            config,
            org,
            key_id,
        }
    }

    async fn teardown(self) {
        let _ =
            admin_api::delete_by_org_api_keys_by_id(&self.config, &self.key_id, &self.org).await;
    }
}

// ---------------------------------------------------------------------------
// High-level client helpers
// ---------------------------------------------------------------------------

/// Build a [`MesaClient`] from environment variables.
pub fn test_client() -> MesaClient {
    let base_url = env::var("MESA_TEST_BASE_URL")
        .unwrap_or_else(|_| "https://depot.mesa.dev/api/v1".to_string());
    let api_key = env::var("MESA_TEST_API_KEY").expect("MESA_TEST_API_KEY must be set");

    let mut builder = MesaClient::builder()
        .with_base_path(base_url)
        .with_api_key(api_key);

    if let Ok(proxy_url) = env::var("MESA_TEST_PROXY") {
        let http_client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(&proxy_url).expect("invalid MESA_TEST_PROXY URL"))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let client = reqwest_middleware::ClientBuilder::new(http_client).build();
        builder = builder.with_client(client);
    }

    builder.build()
}

/// Convenience: create a commit through the high-level client.
pub async fn hl_create_commit(
    client: &MesaClient,
    org: &str,
    repo: &str,
    branch: &str,
    message: &str,
    files: &[(&str, &str)],
) -> CommitResponse {
    let author = CommitAuthor::new("Test Author".to_string(), "test@test.com".to_string());
    let commit_files: Vec<CommitFile> = files
        .iter()
        .map(|(path, content)| CommitFile::Upsert {
            path: (*path).to_string(),
            content: (*content).to_string(),
            encoding: None,
        })
        .collect();

    client
        .org(org)
        .repos()
        .at(repo)
        .commits()
        .create(branch, message, &author, &commit_files, None)
        .await
        .expect("hl_create_commit failed")
}

// ---------------------------------------------------------------------------
// High-level contexts
// ---------------------------------------------------------------------------

/// An empty repository created via the high-level client, deleted on teardown.
pub struct HlRepoContext {
    pub client: MesaClient,
    pub org: String,
    pub repo_name: String,
}

impl AsyncTestContext for HlRepoContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();
        let repo_name = unique_name("hl-repo");

        client
            .org(&org)
            .repos()
            .create(models::PostByOrgReposRequest::new(repo_name.clone()))
            .await
            .expect("failed to create test repo");

        Self {
            client,
            org,
            repo_name,
        }
    }

    async fn teardown(self) {
        let _ = self
            .client
            .org(&self.org)
            .repos()
            .at(&self.repo_name)
            .delete()
            .await;
    }
}

/// A repository with one initial commit (README.md), created and deleted via the
/// high-level client.
pub struct HlRepoWithCommitContext {
    pub client: MesaClient,
    pub org: String,
    pub repo_name: String,
    pub commit_sha: String,
}

impl AsyncTestContext for HlRepoWithCommitContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();
        let repo_name = unique_name("hl-repoc");

        client
            .org(&org)
            .repos()
            .create(models::PostByOrgReposRequest::new(repo_name.clone()))
            .await
            .expect("failed to create test repo");

        let commit = hl_create_commit(
            &client,
            &org,
            &repo_name,
            "main",
            "Initial commit",
            &[("README.md", "# Test Repository")],
        )
        .await;

        Self {
            client,
            org,
            repo_name,
            commit_sha: commit.sha,
        }
    }

    async fn teardown(self) {
        let _ = self
            .client
            .org(&self.org)
            .repos()
            .at(&self.repo_name)
            .delete()
            .await;
    }
}

/// An API key created and revoked via the high-level client.
pub struct HlApiKeyContext {
    pub client: MesaClient,
    pub org: String,
    pub key_id: String,
}

impl AsyncTestContext for HlApiKeyContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();

        let req = models::PostByOrgApiKeysRequest {
            name: Some(Some("hl-integration-test-key".to_string())),
            scopes: Some(vec![
                models::post_by_org_api_keys_request::Scopes::GitColonRead,
                models::post_by_org_api_keys_request::Scopes::RepoColonRead,
            ]),
        };

        let resp = client
            .org(&org)
            .api_keys()
            .create(req)
            .await
            .expect("failed to create test api key");

        let key_id = resp.id.expect("api key id missing");

        Self {
            client,
            org,
            key_id,
        }
    }

    async fn teardown(self) {
        let _ = self
            .client
            .org(&self.org)
            .api_keys()
            .revoke(&self.key_id)
            .await;
    }
}
