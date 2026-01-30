use mesa_dev::Mesa;
use mesa_dev::models::{
    ApiKeyCreated, ApiKeyScope, Author, CommitFile, CommitFileAction, CreateApiKeyRequest,
    CreateCommitRequest, CreateRepoRequest,
};
use test_context::AsyncTestContext;
use uuid::Uuid;

/// Initialize a Mesa client from environment variables.
/// Panics with a helpful message if env vars are missing.
pub fn test_client() -> Mesa {
    let api_key = std::env::var("MESA_TEST_API_KEY")
        .expect("MESA_TEST_API_KEY must be set to run integration tests");
    let org =
        std::env::var("MESA_TEST_ORG").expect("MESA_TEST_ORG must be set to run integration tests");

    // Verify org is non-empty
    assert!(!org.is_empty(), "MESA_TEST_ORG must not be empty");

    let mut builder = Mesa::builder(&api_key);

    if let Ok(base_url) = std::env::var("MESA_TEST_BASE_URL") {
        builder = builder.base_url(&base_url);
    }

    builder.build()
}

/// Get the test org name from the environment.
pub fn test_org() -> String {
    std::env::var("MESA_TEST_ORG").expect("MESA_TEST_ORG must be set")
}

/// Generate a unique repository name for tests to avoid collisions.
pub fn unique_repo_name(prefix: &str) -> String {
    let short_id = &Uuid::new_v4().to_string()[..8];
    format!("test-{prefix}-{short_id}")
}

/// Create a standard test author for commits.
pub fn test_author() -> Author {
    Author {
        name: "Test".to_owned(),
        email: "test@test.com".to_owned(),
        date: None,
    }
}

/// Test context that creates an empty repo and deletes it on teardown.
pub struct RepoContext {
    pub client: Mesa,
    pub org: String,
    pub repo_name: String,
}

impl AsyncTestContext for RepoContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();
        let repo_name = unique_repo_name("ctx");

        client
            .repos(&org)
            .create(&CreateRepoRequest {
                name: repo_name.clone(),
                default_branch: None,
            })
            .await
            .expect("RepoContext: failed to create repo");

        Self {
            client,
            org,
            repo_name,
        }
    }

    async fn teardown(self) {
        let _ = self.client.repos(&self.org).delete(&self.repo_name).await;
    }
}

/// Test context that creates a repo with an initial commit (hello.txt), deletes on teardown.
pub struct RepoWithCommitContext {
    pub client: Mesa,
    pub org: String,
    pub repo_name: String,
    pub commit_sha: String,
}

impl AsyncTestContext for RepoWithCommitContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();
        let repo_name = unique_repo_name("ctxc");

        client
            .repos(&org)
            .create(&CreateRepoRequest {
                name: repo_name.clone(),
                default_branch: None,
            })
            .await
            .expect("RepoWithCommitContext: failed to create repo");

        let commit = client
            .commits(&org, &repo_name)
            .create(&CreateCommitRequest {
                branch: "main".to_owned(),
                message: "Initial commit".to_owned(),
                author: test_author(),
                files: vec![CommitFile {
                    action: CommitFileAction::Upsert,
                    path: "README.md".to_owned(),
                    content: Some("# Test".to_owned()),
                }],
                base_sha: None,
            })
            .await
            .expect("RepoWithCommitContext: failed to create initial commit");

        Self {
            client,
            org,
            repo_name,
            commit_sha: commit.sha,
        }
    }

    async fn teardown(self) {
        let _ = self.client.repos(&self.org).delete(&self.repo_name).await;
    }
}

/// Test context that creates an API key and revokes it on teardown.
pub struct ApiKeyContext {
    pub client: Mesa,
    pub org: String,
    pub created: ApiKeyCreated,
}

impl AsyncTestContext for ApiKeyContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();

        let created = client
            .admin(&org)
            .create_api_key(&CreateApiKeyRequest {
                name: "test-key-e2e".to_owned(),
                scopes: vec![ApiKeyScope::GitRead, ApiKeyScope::RepoRead],
            })
            .await
            .expect("ApiKeyContext: failed to create API key");

        Self {
            client,
            org,
            created,
        }
    }

    async fn teardown(self) {
        let _ = self
            .client
            .admin(&self.org)
            .revoke_api_key(&self.created.id)
            .await;
    }
}
