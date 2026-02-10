use crate::common::{self, HlRepoWithCommitContext};
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_get_diff(ctx: &mut HlRepoWithCommitContext) {
    // Create a second commit so we have two SHAs to diff.
    let second = common::hl_create_commit(
        &ctx.client,
        &ctx.org,
        &ctx.repo_name,
        "main",
        "Second commit",
        &[("hello.txt", "world")],
    )
    .await;

    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .diff()
        .get(&ctx.commit_sha, &second.sha)
        .await
        .unwrap();

    assert!(!resp.files.is_empty());
}
