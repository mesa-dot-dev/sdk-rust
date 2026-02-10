use crate::common;
use mesa_dev::models;

#[tokio::test]
async fn test_hl_delete_repository() {
    let client = common::test_client();
    let org = common::test_org();
    let repo_name = common::unique_name("hl-delete");

    // Create a repo to delete
    let req = models::PostByOrgReposRequest::new(repo_name.clone());
    client.org(&org).repos().create(req).await.unwrap();

    // Delete it
    let resp = client
        .org(&org)
        .repos()
        .at(&repo_name)
        .delete()
        .await
        .unwrap();

    assert!(resp.success);
}
