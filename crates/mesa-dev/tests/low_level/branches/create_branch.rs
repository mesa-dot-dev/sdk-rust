use mesa_dev::low_level::apis::branches_api;
use mesa_dev::models;

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_create_branch(ctx: &mut RepoWithCommitContext) {
    let branch_name = "test-branch";

    let req = models::PostByOrgByRepoBranchesRequest {
        name: Some(branch_name.to_string()),
        from: Some(ctx.commit_sha.clone()),
    };

    let resp = branches_api::post_by_org_by_repo_branches(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        Some(req),
    )
    .await
    .unwrap();

    assert_eq!(resp.name, branch_name);
    assert_eq!(resp.head_sha, ctx.commit_sha);
    assert!(!resp.is_default);
}
