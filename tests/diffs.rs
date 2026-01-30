#![allow(missing_docs, dead_code)]

mod common;

use common::{RepoContext, test_author};
use mesa_dev::models::{CommitFile, CommitFileAction, CreateCommitRequest};
use test_context::test_context;

#[test_context(RepoContext)]
#[tokio::test]
async fn test_get_diff(ctx: &mut RepoContext) {
    let commit1 = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Initial".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "file.txt".to_owned(),
                content: Some("original content".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("first commit");

    let commit2 = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Update file".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "file.txt".to_owned(),
                content: Some("modified content".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("second commit");

    let diff = ctx
        .client
        .diffs(&ctx.org, &ctx.repo_name)
        .get(&commit1.sha, &commit2.sha)
        .await
        .expect("get diff");

    assert_eq!(diff.base, commit1.sha);
    assert_eq!(diff.head, commit2.sha);
    assert!(
        diff.stats.files > 0,
        "Diff should have at least one file changed"
    );
    assert!(!diff.files.is_empty(), "Diff should contain file entries");

    let file = diff.files.iter().find(|f| f.path == "file.txt");
    assert!(file.is_some(), "Diff should include file.txt");
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_diff_with_added_file(ctx: &mut RepoContext) {
    let commit1 = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Initial".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "existing.txt".to_owned(),
                content: Some("existing".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("first commit");

    let commit2 = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Add new file".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "new-file.txt".to_owned(),
                content: Some("new content".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("second commit");

    let diff = ctx
        .client
        .diffs(&ctx.org, &ctx.repo_name)
        .get(&commit1.sha, &commit2.sha)
        .await
        .expect("get diff");

    let added = diff.files.iter().find(|f| f.path == "new-file.txt");
    assert!(added.is_some(), "Diff should show newly added file");
    let status = &added.unwrap().status;
    assert!(
        status == "A" || status == "added",
        "New file should have added status, got: {status}"
    );
}
