use crate::common::HlRepoContext;
use test_context::test_context;

/// Repos without an upstream return a 400 NO_UPSTREAM error.
/// This test verifies the high-level client correctly surfaces that error.
#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_get_sync_status_no_upstream(ctx: &mut HlRepoContext) {
    let result = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .sync()
        .status()
        .await;

    assert!(
        result.is_err(),
        "expected NO_UPSTREAM error for repo without upstream"
    );
}
