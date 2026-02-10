use crate::common::HlRepoContext;
use mesa_dev::low_level::commits::{CommitAuthor, CommitFile};
use test_context::test_context;

#[test_context(HlRepoContext)]
#[tokio::test]
async fn test_hl_create_commit(ctx: &mut HlRepoContext) {
    let author = CommitAuthor::new("Test Author".to_string(), "test@test.com".to_string());
    let files = [CommitFile::Upsert {
        path: "README.md".to_string(),
        content: "# Hello".to_string(),
        encoding: None,
    }];

    let resp = ctx
        .client
        .org(&ctx.org)
        .repos()
        .at(&ctx.repo_name)
        .commits()
        .create("main", "Add readme", &author, &files, None)
        .await
        .unwrap();

    assert!(!resp.sha.is_empty());
    assert_eq!(resp.branch, "main");
    assert_eq!(resp.message, "Add readme");
}
