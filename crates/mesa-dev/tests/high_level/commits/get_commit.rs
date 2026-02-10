use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_get_commit(ctx: &mut HlRepoWithCommitContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .commits()
        .get(&ctx.commit_sha)
        .await
        .unwrap();

    assert_eq!(resp.sha, ctx.commit_sha);
    assert_eq!(resp.message, "Initial commit");
}
