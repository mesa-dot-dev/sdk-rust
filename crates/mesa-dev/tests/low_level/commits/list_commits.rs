use mesa_dev::low_level::apis::commits_api;

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_list_commits(ctx: &mut RepoWithCommitContext) {
    let resp = commits_api::get_by_org_by_repo_commits(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    assert!(!resp.commits.is_empty());
    assert_eq!(resp.commits[0].message, Some("Initial commit".to_string()));
}
