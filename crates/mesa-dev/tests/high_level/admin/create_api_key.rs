use crate::common;
use mesa_dev::models;

#[tokio::test]
async fn test_hl_create_api_key() {
    let client = common::test_client();
    let org = common::test_org();

    let req = models::PostByOrgApiKeysRequest {
        name: Some(Some("hl-test-create-key".to_string())),
        scopes: Some(vec![
            models::post_by_org_api_keys_request::Scopes::GitColonRead,
            models::post_by_org_api_keys_request::Scopes::RepoColonRead,
        ]),
    };

    let resp = client.org(&org).api_keys().create(req).await.unwrap();

    let key_id = resp.id.expect("api key id missing");
    assert!(!key_id.is_empty());

    // Cleanup: revoke the created key
    let _ = client.org(&org).api_keys().revoke(&key_id).await;
}
