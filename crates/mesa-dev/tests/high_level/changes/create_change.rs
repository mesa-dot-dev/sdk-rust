use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_create_change(ctx: &mut HlRepoWithCommitContext) {
    let org = ctx.client.org(&ctx.org);
    let repo = org.repos().at(&ctx.repo_name);
    let change_client = repo.change().await.unwrap();

    let change = change_client
        .create_from_ref("refs/heads/main")
        .await
        .unwrap();

    assert!(change.id.is_some(), "change should have an id");
    assert!(
        change.current_commit_oid.is_some(),
        "change should have a current commit OID"
    );
}
