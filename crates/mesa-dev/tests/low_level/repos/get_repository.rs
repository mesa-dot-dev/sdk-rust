use mesa_dev::low_level::apis::repos_api;

use crate::common::RepoContext;
use test_context::test_context;

#[test_context(RepoContext)]
#[tokio::test]
async fn test_get_repository(ctx: &mut RepoContext) {
    let resp = repos_api::get_by_org_by_repo(&ctx.config, &ctx.org, Some(&ctx.repo_name))
        .await
        .unwrap();

    assert_eq!(resp.name, ctx.repo_name);
    assert_eq!(resp.org, ctx.org);
    assert_eq!(resp.default_branch, "main");
}
