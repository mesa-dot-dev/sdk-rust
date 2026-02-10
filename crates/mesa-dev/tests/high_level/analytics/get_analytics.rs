use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_get_analytics(ctx: &mut HlRepoWithCommitContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .analytics()
        .get(None)
        .await
        .unwrap();

    // Just verify the call succeeds and returns a valid response.
    let _ = resp;
}
