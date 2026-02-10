use mesa_dev::low_level::apis::admin_api;

use crate::common;

#[tokio::test]
async fn test_list_api_keys() {
    let config = common::test_config();
    let org = common::test_org();

    let resp = admin_api::get_by_org_api_keys(&config, &org).await.unwrap();

    // The org should have at least one key (the one we're using to authenticate).
    assert!(!resp.api_keys.is_empty());
}
