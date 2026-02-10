use crate::common::HlRepoWithCommitContext;
use futures::TryStreamExt;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_list_commits(ctx: &mut HlRepoWithCommitContext) {
    let commits: Vec<_> = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .commits()
        .list(None, None)
        .try_collect()
        .await
        .unwrap();

    assert!(!commits.is_empty());
    assert!(commits
        .iter()
        .any(|c| c.sha.as_deref() == Some(ctx.commit_sha.as_str())));
}
