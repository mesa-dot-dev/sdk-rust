use crate::common::HlRepoContext;
use test_context::test_context;

#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_list_webhooks(ctx: &mut HlRepoContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .webhooks()
        .list()
        .await
        .unwrap();

    // New repo has no webhooks.
    assert!(resp.webhooks.is_empty());
}
