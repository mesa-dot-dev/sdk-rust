use mesa_dev::low_level::apis::repos_api;

use crate::common;

#[tokio::test]
async fn test_list_repositories() {
    let config = common::test_config();
    let org = common::test_org();

    let resp = repos_api::get_by_org_repos(&config, &org, None, None)
        .await
        .unwrap();

    // The org should have at least some repos.
    assert!(!resp.repos.is_empty());
}
