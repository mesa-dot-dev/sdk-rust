use crate::common::HlRepoContext;
use test_context::test_context;

#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_get_repository(ctx: &mut HlRepoContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .get()
        .await
        .unwrap();

    assert_eq!(resp.name, ctx.repo_name);
}
