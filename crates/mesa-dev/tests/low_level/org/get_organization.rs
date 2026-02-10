use mesa_dev::low_level::apis::org_api;

use crate::common;

#[tokio::test]
async fn test_get_organization() {
    let config = common::test_config();
    let org = common::test_org();

    let resp = org_api::get_by_org(&config, &org).await.unwrap();

    assert!(resp.num_repos >= 0);
    assert!(resp.created_at.is_some());
}
