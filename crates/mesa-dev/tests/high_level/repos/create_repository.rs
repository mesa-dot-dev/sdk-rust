use crate::common;
use mesa_dev::models;

#[tokio::test]
async fn test_hl_create_repository() {
    let client = common::test_client();
    let org = common::test_org();
    let repo_name = common::unique_name("hl-create");

    let req = models::PostByOrgReposRequest::new(repo_name.clone());
    let resp = client.org(&org).repos().create(req).await.unwrap();

    assert_eq!(resp.name, repo_name);

    // Cleanup
    let _ = client.org(&org).repos().at(&repo_name).delete().await;
}
