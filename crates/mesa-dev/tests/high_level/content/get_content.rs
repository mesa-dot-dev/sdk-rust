use crate::common::HlRepoWithCommitContext;
use mesa_dev::low_level::content::Content;
use test_context::test_context;

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_get_content_root(ctx: &mut HlRepoWithCommitContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .content()
        .get(None, None, None)
        .await
        .unwrap();

    assert!(matches!(resp, Content::Dir(_)));
}

#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_hl_get_content_file(ctx: &mut HlRepoWithCommitContext) {
    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .content()
        .get(None, Some("README.md"), None)
        .await
        .unwrap();

    match resp {
        Content::File(file) => {
            assert_eq!(file.name.as_deref(), Some("README.md"));
            assert_eq!(file.path.as_deref(), Some("README.md"));
        }
        other => panic!("expected Content::File, got {other:?}"),
    }
}
