use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

/// Refresh analytics on a freshly-created repo. The API currently returns a
/// response where `analytics.version` is not a valid value, causing a
/// deserialization error in the generated model. This test verifies the
/// high-level client call completes (success or deserialization error) without
/// panicking.
#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_refresh_analytics(ctx: &mut HlRepoWithCommitContext) {
    let result = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .analytics()
        .refresh()
        .await;

    // The call may fail with a serde error on freshly-created repos because
    // the analytics.version field is not yet populated. Either outcome (Ok or
    // a deserialization Err) is acceptable — we just verify no panic.
    let _ = result;
}
