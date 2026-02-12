use mesa_dev::low_level::apis::commits_api;

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_get_commit(ctx: &mut RepoWithCommitContext) {
    let resp = commits_api::get_by_org_by_repo_commits_by_sha(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        Some(&ctx.commit_sha),
    )
    .await
    .unwrap();

    assert_eq!(resp.sha, ctx.commit_sha);
    assert_eq!(resp.message, "Initial commit");
    assert_eq!(resp.author.name, "Test Author");
    assert_eq!(resp.author.email, "test@test.com");
}
