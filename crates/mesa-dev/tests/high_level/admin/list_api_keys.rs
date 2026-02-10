use crate::common;

#[tokio::test]
async fn test_hl_list_api_keys() {
    let client = common::test_client();
    let org = common::test_org();

    let resp = client.org(&org).api_keys().list().await.unwrap();

    // The org should always have at least one key (the one we're testing with).
    assert!(!resp.api_keys.is_empty());
}
