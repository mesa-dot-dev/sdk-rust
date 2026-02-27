use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_create_file(ctx: &mut HlRepoWithCommitContext) {
    let org = ctx.client.org(&ctx.org);
    let repo = org.repos().at(&ctx.repo_name);
    let change_client = repo.change().await.unwrap();

    let change = change_client
        .create_from_ref("refs/heads/main")
        .await
        .unwrap();
    let change_id = change.id.unwrap();

    let resp = change_client
        .create_file(&change_id, "hello.txt", b"Hello, world!", None)
        .await
        .unwrap();

    assert!(
        resp.applied_ops_count > 0,
        "should have applied at least one op"
    );

    let snapshot = change_client
        .snapshot(&change_id, "Add hello.txt")
        .await
        .unwrap();

    assert!(
        snapshot.commit_oid.is_some(),
        "snapshot should produce a commit"
    );
}
