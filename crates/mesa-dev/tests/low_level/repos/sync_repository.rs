use mesa_dev::low_level::apis::repos_api;
use mesa_dev::models;

use crate::common;

#[tokio::test]
async fn test_sync_repository() {
    let config = common::test_config();
    let org = common::test_org();
    let name = common::unique_name("sync-trigger");

    // Create repo with upstream
    let req = models::PostByOrgReposRequest {
        name: name.clone(),
        default_branch: None,
        upstream: Some(models::PostByOrgReposRequestUpstream::new(
            "https://github.com/octocat/Hello-World.git".to_string(),
        )),
    };
    repos_api::post_by_org_repos(&config, &org, Some(req))
        .await
        .unwrap();

    let resp = repos_api::post_by_org_by_repo_sync(&config, &org, &name)
        .await
        .unwrap();

    assert!(resp.success);

    // Cleanup
    let _ = repos_api::delete_by_org_by_repo(&config, &org, &name).await;
}
