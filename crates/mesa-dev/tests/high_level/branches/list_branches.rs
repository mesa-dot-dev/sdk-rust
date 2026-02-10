use crate::common::HlRepoWithCommitContext;
use futures::TryStreamExt;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_list_branches(ctx: &mut HlRepoWithCommitContext) {
    let branches: Vec<_> = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .branches()
        .list(None)
        .try_collect()
        .await
        .unwrap();

    assert!(!branches.is_empty());
    // The default branch "main" should be present.
    assert!(branches.iter().any(|b| b.name.as_deref() == Some("main")));
}
