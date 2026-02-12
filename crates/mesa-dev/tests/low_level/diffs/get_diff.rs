use mesa_dev::low_level::apis::diffs_api;

use crate::common::{self, RepoWithCommitContext};
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_get_diff(ctx: &mut RepoWithCommitContext) {
    // Create a second commit that modifies README.md
    let commit2 = common::create_commit(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        "main",
        "Update readme",
        &[("README.md", "# Updated Content")],
    )
    .await;

    let resp = diffs_api::get_by_org_by_repo_diff(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        Some(&ctx.commit_sha),
        Some(&commit2.sha),
    )
    .await
    .unwrap();

    assert!(resp.stats.files >= 1.0);
    assert!(resp.stats.changes >= 1.0);
    assert!(!resp.files.is_empty());
}
