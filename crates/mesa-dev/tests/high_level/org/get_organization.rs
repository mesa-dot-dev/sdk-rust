use crate::common;

#[tokio::test]
async fn test_hl_get_organization() {
    let client = common::test_client();
    let org = common::test_org();

    let resp = client.org(&org).get().await.unwrap();

    // The response doesn't carry the org name, but it should have a non-negative
    // repo count, confirming the call succeeded.
    assert!(resp.num_repos >= 0);
}
