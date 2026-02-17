use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_modify_file(ctx: &mut HlRepoWithCommitContext) {
    let org = ctx.client.org(&ctx.org);
    let repo = org.repos().at(&ctx.repo_name);
    let change_client = repo.change().await.unwrap();

    let change = change_client
        .create_from_ref("refs/heads/main")
        .await
        .unwrap();
    let change_id = change.id.unwrap();

    let resp = change_client
        .modify_file(&change_id, "README.md", b"# Updated Repository")
        .await
        .unwrap();

    assert!(resp.applied_ops_count > 0);

    let snapshot = change_client
        .snapshot(&change_id, "Update README.md")
        .await
        .unwrap();

    assert!(snapshot.commit_oid.is_some());
}
