use crate::common::HlRepoWithCommitContext;
use futures::TryStreamExt;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
#[ignore = "commits list endpoint not yet available for git-pushed repos"]
async fn test_hl_get_diff(ctx: &mut HlRepoWithCommitContext) {
    // Grab two commits from the seeded repo to diff.
    let commits: Vec<_> = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .commits()
        .list(None, Some(2))
        .try_collect()
        .await
        .expect("list commits failed");

    assert!(commits.len() >= 2, "need at least 2 commits to diff");

    let newer = commits[0].sha.as_deref().unwrap();
    let older = commits[1].sha.as_deref().unwrap();

    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .diff()
        .get(older, newer)
        .await
        .unwrap();

    assert!(!resp.files.is_empty());
}
