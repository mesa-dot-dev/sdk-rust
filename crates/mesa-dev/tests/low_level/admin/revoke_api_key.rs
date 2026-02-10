use mesa_dev::low_level::apis::admin_api;

use crate::common::{self, ApiKeyContext};
use test_context::test_context;

#[test_context(ApiKeyContext)]
#[tokio::test]
async fn test_revoke_api_key(ctx: &mut ApiKeyContext) {
    let resp = admin_api::delete_by_org_api_keys_by_id(&ctx.config, &ctx.key_id, &ctx.org)
        .await
        .unwrap();

    assert!(resp.success);
}
