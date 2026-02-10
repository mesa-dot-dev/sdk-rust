use crate::common::HlRepoContext;
use mesa_dev::models;
use test_context::test_context;

#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_update_repository(ctx: &mut HlRepoContext) {
    let new_name = format!("{}-renamed", ctx.repo_name);
    let req = models::PatchByOrgByRepoRequest {
        name: Some(new_name.clone()),
        default_branch: None,
        upstream: None,
    };

    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .update(req)
        .await
        .unwrap();

    assert_eq!(resp.name, new_name);

    // Update ctx.repo_name so teardown deletes the renamed repo.
    ctx.repo_name = new_name;
}
