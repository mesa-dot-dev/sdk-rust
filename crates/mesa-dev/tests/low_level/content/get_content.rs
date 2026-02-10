use mesa_dev::low_level::content::{self, Content};

use crate::common::RepoWithCommitContext;
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_get_content_root(ctx: &mut RepoWithCommitContext) {
    let resp = content::get_content(&ctx.config, &ctx.org, &ctx.repo_name, None, None, None)
        .await
        .unwrap();

    assert!(matches!(resp, Content::Dir(_)));
}

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_get_content_file(ctx: &mut RepoWithCommitContext) {
    let resp = content::get_content(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        None,
        Some("README.md"),
        None,
    )
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
