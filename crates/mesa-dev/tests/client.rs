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
use mesa_dev::MesaClient;
use std::env;

/// Smoke test: exercises the MesaClient -> OrgClient -> RepoClient chain.
#[tokio::test]
async fn client_round_trip() {
    let config = common::test_config();
    let org_name = common::test_org();
    let repo_name = common::unique_name("client");

    // Create and seed the repo via git push.
    let commit_sha = common::create_seeded_repo(&config, &org_name, &repo_name).await;
    assert!(!commit_sha.is_empty());

    let grpc_endpoint = env::var("MESA_TEST_GRPC_ENDPOINT")
        .unwrap_or_else(|_| mesa_dev::DEFAULT_GRPC_ENDPOINT.to_string());
    let grpc_channel = tonic::transport::Channel::from_shared(grpc_endpoint)
        .expect("invalid gRPC endpoint")
        .connect_lazy();
    let client = MesaClient::from_configuration(config, grpc_channel);
    let org = client.org(&org_name);
    let repo = org.repos().at(&repo_name);

    // List branches
    let branches: Vec<_> = repo
        .branches()
        .list(None)
        .try_collect()
        .await
        .expect("list branches failed");
    assert!(!branches.is_empty());

    // Get content (planventure has a README.md)
    let content = repo
        .content()
        .get(None, Some("README.md"), None)
        .await
        .expect("get content failed");
    assert!(matches!(
        content,
        mesa_dev::low_level::content::Content::File(_)
    ));

    // Cleanup
    repo.delete().await.expect("delete repo failed");
}
