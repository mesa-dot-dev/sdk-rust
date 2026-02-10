use crate::common::HlRepoContext;
use mesa_dev::models;
use test_context::test_context;

#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_create_webhook(ctx: &mut HlRepoContext) {
    let req = models::PostByOrgByRepoWebhooksRequest {
        url: "https://example.com/webhook".to_string(),
        secret: Some("test-secret".to_string()),
        events: Some(vec![
            models::post_by_org_by_repo_webhooks_request::Events::Push,
        ]),
        branches: None,
        globs: None,
    };

    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .webhooks()
        .create(req)
        .await
        .unwrap();

    assert_eq!(resp.url.as_deref(), Some("https://example.com/webhook"));
}
