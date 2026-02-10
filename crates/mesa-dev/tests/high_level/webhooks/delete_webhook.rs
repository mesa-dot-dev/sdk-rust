use crate::common::HlRepoContext;
use mesa_dev::models;
use test_context::test_context;

#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_delete_webhook(ctx: &mut HlRepoContext) {
    // Create a webhook first
    let req = models::PostByOrgByRepoWebhooksRequest {
        url: "https://example.com/hook-to-delete".to_string(),
        secret: Some("test-secret".to_string()),
        events: Some(vec![
            models::post_by_org_by_repo_webhooks_request::Events::Push,
        ]),
        branches: None,
        globs: None,
    };

    let created = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .webhooks()
        .create(req)
        .await
        .unwrap();

    let webhook_id = created.id.expect("webhook id missing");

    // Delete it
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .webhooks()
        .delete(&webhook_id)
        .await
        .unwrap();

    assert!(resp.success);
}
