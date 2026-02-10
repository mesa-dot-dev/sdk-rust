use crate::common::HlRepoWithCommitContext;
use mesa_dev::models;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_delete_branch(ctx: &mut HlRepoWithCommitContext) {
    // Create a branch first
    let branch_name = "branch-to-delete";
    let req = models::PostByOrgByRepoBranchesRequest {
        name: branch_name.to_string(),
        from: ctx.commit_sha.clone(),
    };
    ctx.client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .branches()
        .create(req)
        .await
        .unwrap();

    // Delete it
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .branches()
        .delete(branch_name)
        .await
        .unwrap();

    assert!(resp.success);
}
