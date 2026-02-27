use crate::common::HlRepoWithCommitContext;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_delete_file(ctx: &mut HlRepoWithCommitContext) {
    let org = ctx.client.org(&ctx.org);
    let repo = org.repos().at(&ctx.repo_name);
    let change_client = repo.change().await.unwrap();

    let change = change_client
        .create_from_ref("refs/heads/main")
        .await
        .unwrap();
    let change_id = change.id.unwrap();

    let resp = change_client
        .delete_path(&change_id, "README.md", false)
        .await
        .unwrap();

    assert!(resp.applied_ops_count > 0);

    let snapshot = change_client
        .snapshot(&change_id, "Delete README.md")
        .await
        .unwrap();

    assert!(snapshot.commit_oid.is_some());
}
