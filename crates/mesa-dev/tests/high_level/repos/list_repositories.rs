use crate::common;
use futures::TryStreamExt;

#[tokio::test]
async fn test_hl_list_repositories() {
    let client = common::test_client();
    let org = common::test_org();

    let repos: Vec<_> = client
        .org(&org)
        .repos()
        .list(None)
        .try_collect()
        .await
        .unwrap();

    // The org should have at least some repos.
    assert!(!repos.is_empty());
}
