use mesa_dev::low_level::apis::repos_api;
use mesa_dev::models;

use crate::common;

#[tokio::test]
async fn test_create_repository() {
    let config = common::test_config();
    let org = common::test_org();
    let name = common::unique_name("create");

    let req = models::PostByOrgReposRequest::new(name.clone());
    let resp = repos_api::post_by_org_repos(&config, &org, Some(req))
        .await
        .unwrap();

    assert_eq!(resp.name, name);
    assert_eq!(resp.org, org);
    assert_eq!(resp.default_branch, "main");

    // Cleanup
    let _ = repos_api::delete_by_org_by_repo(&config, &org, &name).await;
}
