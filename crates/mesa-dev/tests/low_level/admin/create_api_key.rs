use mesa_dev::low_level::apis::admin_api;
use mesa_dev::models;
use mesa_dev::models::post_by_org_api_keys_request::Scopes;

use crate::common;

#[tokio::test]
async fn test_create_api_key() {
    let config = common::test_config();
    let org = common::test_org();

    let req = models::PostByOrgApiKeysRequest {
        name: Some(Some("ci-create-test".to_string())),
        scopes: Some(vec![Scopes::GitColonRead, Scopes::RepoColonRead]),
    };

    let resp = admin_api::post_by_org_api_keys(&config, &org, Some(req))
        .await
        .unwrap();

    // The secret key is only returned at creation time.
    assert!(resp.key.is_some(), "key secret should be returned");
    assert!(resp.id.is_some(), "key id should be returned");
    assert!(resp.scopes.contains(&"git:read".to_string()));
    assert!(resp.scopes.contains(&"repo:read".to_string()));

    // Cleanup: revoke the key we just created.
    let key_id = resp.id.as_ref().unwrap();
    admin_api::delete_by_org_api_keys_by_id(&config, key_id, &org)
        .await
        .unwrap();
}
