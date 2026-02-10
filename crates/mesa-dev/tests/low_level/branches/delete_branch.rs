use mesa_dev::low_level::apis::branches_api;
use mesa_dev::models;

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_delete_branch(ctx: &mut RepoWithCommitContext) {
    // Create a branch to delete
    let branch_name = "branch-to-delete";
    let req = models::PostByOrgByRepoBranchesRequest {
        name: branch_name.to_string(),
        from: ctx.commit_sha.clone(),
    };
    branches_api::post_by_org_by_repo_branches(&ctx.config, &ctx.org, &ctx.repo_name, Some(req))
        .await
        .unwrap();

    // Delete it
    let resp = branches_api::delete_by_org_by_repo_branches_by_branch(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        branch_name,
    )
    .await
    .unwrap();

    assert!(resp.success);
}
