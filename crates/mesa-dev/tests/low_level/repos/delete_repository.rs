use mesa_dev::low_level::apis::repos_api;
use mesa_dev::models;

use crate::common;

#[tokio::test]
async fn test_delete_repository() {
    let config = common::test_config();
    let org = common::test_org();
    let name = common::unique_name("delete");

    // Create
    let req = models::PostByOrgReposRequest::new(name.clone());
    repos_api::post_by_org_repos(&config, &org, Some(req))
        .await
        .unwrap();

    // Delete
    let resp = repos_api::delete_by_org_by_repo(&config, &org, &name)
        .await
        .unwrap();

    assert!(resp.success);

    // Verify it's gone
    let get_result = repos_api::get_by_org_by_repo(&config, &org, Some(&name)).await;
    assert!(get_result.is_err());
}
