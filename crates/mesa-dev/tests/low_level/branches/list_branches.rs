use mesa_dev::low_level::apis::branches_api;

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_list_branches(ctx: &mut RepoWithCommitContext) {
    let resp = branches_api::get_by_org_by_repo_branches(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        None,
        None,
    )
    .await
    .unwrap();

    assert!(!resp.branches.is_empty());

    let main = resp
        .branches
        .iter()
        .find(|b| b.name.as_deref() == Some("main"));
    assert!(main.is_some(), "main branch should exist");
    assert!(main.unwrap().is_default);
}
