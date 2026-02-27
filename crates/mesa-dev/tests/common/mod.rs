use mesa_dev::low_level::apis::configuration::Configuration;
use mesa_dev::low_level::apis::{admin_api, branches_api, repos_api};
use mesa_dev::models;
use mesa_dev::MesaClient;
use std::env;
use test_context::AsyncTestContext;
use uuid::Uuid;

/// Public upstream used to seed repositories with commits via git push.
const SEED_REPO_URL: &str = "https://github.com/github-samples/planventure.git";

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
// Git-push seed helper
// ---------------------------------------------------------------------------

/// Create a repository, clone a public upstream, and push it to the depot so
/// the repo is populated with commits. Returns the `head_oid` of `main`.
///
/// This replaces the old `create_commit` helper — the commit-creation API
/// endpoint no longer exists, so we seed data via `git push` instead.
pub async fn create_seeded_repo(config: &Configuration, org: &str, name: &str) -> String {
    // 1. Create the empty repo via the API.
    let req = models::PostByOrgReposRequest::new(name.to_string());
    repos_api::post_by_org_repos(config, org, Some(req))
        .await
        .expect("failed to create test repo");

    // 2. Clone the seed repo into a temp dir and push to the depot.
    let api_key = config
        .bearer_access_token
        .as_deref()
        .expect("missing API key");
    let git_host =
        env::var("MESA_TEST_GIT_HOST").unwrap_or_else(|_| "depot.mesa.dev".to_string());
    let remote_url = format!("https://{api_key}@{git_host}/{org}/{name}.git");

    let tmp = tempfile::tempdir().expect("failed to create tempdir");
    let tmp_path = tmp.path();

    let clone_status = std::process::Command::new("git")
        .args(["clone", "--quiet", SEED_REPO_URL, "."])
        .current_dir(tmp_path)
        .status()
        .expect("failed to spawn git clone");
    assert!(clone_status.success(), "git clone failed");

    let push_status = std::process::Command::new("git")
        .args(["push", "--quiet", &remote_url, "main"])
        .current_dir(tmp_path)
        .status()
        .expect("failed to spawn git push");
    assert!(push_status.success(), "git push failed");

    // 3. Wait for the depot to index the push.
    //    Both the branches and commits endpoints need time after a push.
    let mut head_oid = None;
    for _ in 0..30 {
        if let Ok(resp) =
            branches_api::get_by_org_by_repo_branches(config, org, name, None, Some(1)).await
        {
            if let Some(branch) = resp.branches.first() {
                head_oid = branch.head_oid.clone();
                break;
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
    let head_oid = head_oid.expect("timed out waiting for branches after git push");

    head_oid
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

/// A repository seeded with commits from a public upstream, deleted on teardown.
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

        let commit_sha = create_seeded_repo(&config, &org, &repo_name).await;

        Self {
            config,
            org,
            repo_name,
            commit_sha,
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
    let grpc_endpoint = env::var("MESA_TEST_GRPC_ENDPOINT")
        .unwrap_or_else(|_| mesa_dev::DEFAULT_GRPC_ENDPOINT.to_string());

    let mut builder = MesaClient::builder()
        .with_base_path(base_url)
        .with_api_key(api_key)
        .with_grpc_endpoint(grpc_endpoint);

    if let Ok(proxy_url) = env::var("MESA_TEST_PROXY") {
        let http_client = reqwest::Client::builder()
            .proxy(reqwest::Proxy::all(&proxy_url).expect("invalid MESA_TEST_PROXY URL"))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
        let client = reqwest_middleware::ClientBuilder::new(http_client).build();
        builder = builder.with_client(client);
    }

    builder.build().expect("failed to build MesaClient")
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

/// A repository seeded with commits from a public upstream, using the
/// high-level client. Deleted on teardown.
pub struct HlRepoWithCommitContext {
    pub client: MesaClient,
    pub org: String,
    pub repo_name: String,
    pub commit_sha: String,
}

impl AsyncTestContext for HlRepoWithCommitContext {
    async fn setup() -> Self {
        let client = test_client();
        let config = test_config();
        let org = test_org();
        let repo_name = unique_name("hl-repoc");

        let commit_sha = create_seeded_repo(&config, &org, &repo_name).await;

        Self {
            client,
            org,
            repo_name,
            commit_sha,
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
