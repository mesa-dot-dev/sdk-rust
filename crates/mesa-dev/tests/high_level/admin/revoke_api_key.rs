use crate::common::HlApiKeyContext;
use test_context::test_context;

#[test_context(HlApiKeyContext)]
#[tokio::test]
async fn test_hl_revoke_api_key(ctx: &mut HlApiKeyContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .api_keys()
        .revoke(&ctx.key_id)
        .await
        .unwrap();

    assert!(resp.success);

    // Clear key_id so teardown doesn't try to revoke again.
    ctx.key_id = String::new();
}
