use crate::common::HlRepoWithCommitContext;
use mesa_dev::models;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_create_branch(ctx: &mut HlRepoWithCommitContext) {
    let branch_name = "test-branch";
    let req = models::PostByOrgByRepoBranchesRequest {
        name: Some(branch_name.to_string()),
        from: Some(ctx.commit_sha.clone()),
    };

    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .branches()
        .create(req)
        .await
        .unwrap();

    assert_eq!(resp.name, branch_name);
    assert_eq!(resp.head_sha, ctx.commit_sha);
    assert!(!resp.is_default);
}
