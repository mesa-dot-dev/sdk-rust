#![allow(
    missing_docs,
    dead_code,
    unused_imports,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::doc_markdown
)]

#[path = "common/mod.rs"]
mod common;

use futures::TryStreamExt;
use mesa_dev::low_level::commits::{CommitAuthor, CommitFile};
use mesa_dev::MesaClient;

/// Smoke test: exercises the MesaClient -> OrgClient -> RepoClient chain.
#[tokio::test]
async fn client_round_trip() {
    let config = common::test_config();
    let org_name = common::test_org();
    let repo_name = common::unique_name("client");

    let client = MesaClient::from_configuration(config);
    let org = client.org(&org_name);

    // Create repo
    let create_resp = org
        .repos()
        .create(mesa_dev::models::PostByOrgReposRequest::new(
            repo_name.clone(),
        ))
        .await
        .expect("create repo failed");
    assert_eq!(create_resp.name, repo_name);

    let repo = org.repos().at(&repo_name);

    // Create a commit via the client
    let author = CommitAuthor::new("Test".to_string(), "test@test.com".to_string());
    let files = vec![CommitFile::Upsert {
        path: "hello.txt".to_string(),
        content: "world".to_string(),
        encoding: None,
    }];
    let commit = repo
        .commits()
        .create("main", "test commit", &author, &files, None)
        .await
        .expect("create commit failed");
    assert!(!commit.sha.is_empty());

    // List branches
    let branches: Vec<_> = repo
        .branches()
        .list(None)
        .try_collect()
        .await
        .expect("list branches failed");
    assert!(!branches.is_empty());

    // Get content
    let content = repo
        .content()
        .get(None, Some("hello.txt"), None)
        .await
        .expect("get content failed");
    assert!(matches!(
        content,
        mesa_dev::low_level::content::Content::File(_)
    ));

    // Cleanup
    repo.delete().await.expect("delete repo failed");
}
