use mesa_dev::low_level::apis::{commits_api, diffs_api};

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
#[ignore = "commits list endpoint not yet available for git-pushed repos"]
async fn test_get_diff(ctx: &mut RepoWithCommitContext) {
    // Grab two commits from the seeded repo to diff.
    let listing = commits_api::get_by_org_by_repo_commits(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        None,
        Some(2),
        None,
    )
    .await
    .expect("list commits failed");

    assert!(
        listing.commits.len() >= 2,
        "need at least 2 commits to diff"
    );

    let newer = listing.commits[0].sha.as_deref().unwrap();
    let older = listing.commits[1].sha.as_deref().unwrap();

    let resp = diffs_api::get_by_org_by_repo_diff(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        Some(older),
        Some(newer),
    )
    .await
    .unwrap();

    assert!(!resp.files.is_empty());
}
