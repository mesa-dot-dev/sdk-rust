use crate::common::RepoContext;
use mesa_dev::low_level::commits::{self, CommitAuthor, CommitFile};
use test_context::test_context;

#[test_context(RepoContext)]
#[tokio::test]
async fn test_create_commit(ctx: &mut RepoContext) {
    let author = CommitAuthor::new("Test Author".to_string(), "test@test.com".to_string());
    let files = [CommitFile::Upsert {
        path: "README.md".to_string(),
        content: "# Hello".to_string(),
        encoding: None,
    }];

    let resp = commits::create_commit(
        &ctx.config,
        &ctx.org,
        &ctx.repo_name,
        "main",
        "Add readme",
        &author,
        &files,
        None,
    )
    .await
    .expect("create_commit failed");

    assert!(!resp.sha.is_empty());
    assert_eq!(resp.branch, "main");
    assert_eq!(resp.message, "Add readme");
}
