#![allow(missing_docs, dead_code)]

mod common;

use std::panic::AssertUnwindSafe;

use common::{
    RepoContext, RepoWithCommitContext, test_author, test_client, test_org, unique_repo_name,
};
use futures::FutureExt;
use mesa_dev::models::{
    CommitFile, CommitFileAction, CreateBranchRequest, CreateCommitRequest, CreateRepoRequest,
    PaginationParams,
};
use mesa_dev::resources::ListCommitsParams;
use test_context::test_context;

#[test_context(RepoContext)]
#[tokio::test]
async fn test_create_commit(ctx: &mut RepoContext) {
    let commit = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Add hello.txt".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "hello.txt".to_owned(),
                content: Some("Hello, world!".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("create commit");

    assert!(!commit.sha.is_empty());
    assert_eq!(commit.message, "Add hello.txt");
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_create_multiple_files_in_commit(ctx: &mut RepoContext) {
    let commit = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Add multiple files".to_owned(),
            author: test_author(),
            files: vec![
                CommitFile {
                    action: CommitFileAction::Upsert,
                    path: "file1.txt".to_owned(),
                    content: Some("Content 1".to_owned()),
                },
                CommitFile {
                    action: CommitFileAction::Upsert,
                    path: "dir/file2.txt".to_owned(),
                    content: Some("Content 2".to_owned()),
                },
            ],
            base_sha: None,
        })
        .await
        .expect("create commit");

    assert!(!commit.sha.is_empty());
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_get_commit(ctx: &mut RepoContext) {
    let created = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Commit to fetch".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "test.txt".to_owned(),
                content: Some("test".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("create commit");

    let fetched = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .get(&created.sha)
        .await
        .expect("get commit");

    assert_eq!(fetched.sha, created.sha);
    assert_eq!(fetched.message, "Commit to fetch");
    assert_eq!(fetched.author.name, "Test");
    assert_eq!(fetched.author.email, "test@test.com");
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_list_commits(ctx: &mut RepoContext) {
    for i in 0..2 {
        ctx.client
            .commits(&ctx.org, &ctx.repo_name)
            .create(&CreateCommitRequest {
                branch: "main".to_owned(),
                message: format!("Commit {i}"),
                author: test_author(),
                files: vec![CommitFile {
                    action: CommitFileAction::Upsert,
                    path: format!("file{i}.txt"),
                    content: Some(format!("content {i}")),
                }],
                base_sha: None,
            })
            .await
            .expect("create commit");
    }

    let response = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .list(&ListCommitsParams {
            pagination: PaginationParams {
                cursor: None,
                limit: Some(10),
            },
            ref_: None,
        })
        .await
        .expect("list commits");

    assert!(
        response.commits.len() >= 2,
        "Expected at least 2 commits, got {}",
        response.commits.len()
    );
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_list_all_commits(ctx: &mut RepoContext) {
    for i in 0..3 {
        ctx.client
            .commits(&ctx.org, &ctx.repo_name)
            .create(&CreateCommitRequest {
                branch: "main".to_owned(),
                message: format!("Commit {i}"),
                author: test_author(),
                files: vec![CommitFile {
                    action: CommitFileAction::Upsert,
                    path: format!("file{i}.txt"),
                    content: Some(format!("content {i}")),
                }],
                base_sha: None,
            })
            .await
            .expect("create commit");
    }

    let all = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .list_all(None)
        .collect()
        .await
        .expect("list all commits");

    assert!(
        all.len() >= 3,
        "Expected at least 3 commits, got {}",
        all.len()
    );
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_delete_file_in_commit(ctx: &mut RepoContext) {
    ctx.client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Add file".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "to-delete.txt".to_owned(),
                content: Some("delete me".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("create file commit");

    let commit = ctx
        .client
        .commits(&ctx.org, &ctx.repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Delete file".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Delete,
                path: "to-delete.txt".to_owned(),
                content: None,
            }],
            base_sha: None,
        })
        .await
        .expect("delete file commit");

    assert!(!commit.sha.is_empty());
}

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_list_commits_raw_pagination(ctx: &mut RepoWithCommitContext) {
    // Context provides 1 initial commit; create 2 more for 3 total
    let mut commit_shas: Vec<String> = vec![ctx.commit_sha.clone()];
    for i in 0..2 {
        let commit = ctx
            .client
            .commits(&ctx.org, &ctx.repo_name)
            .create(&CreateCommitRequest {
                branch: "main".to_owned(),
                message: format!("Page commit {i}"),
                author: test_author(),
                files: vec![CommitFile {
                    action: CommitFileAction::Upsert,
                    path: format!("page{i}.txt"),
                    content: Some(format!("content {i}")),
                }],
                base_sha: None,
            })
            .await
            .expect("Failed to create commit");
        commit_shas.push(commit.sha);
    }

    let mut collected_shas: Vec<String> = Vec::new();
    let mut cursor: Option<String> = None;
    let mut pages = 0;

    loop {
        let response = ctx
            .client
            .commits(&ctx.org, &ctx.repo_name)
            .list(&ListCommitsParams {
                pagination: PaginationParams {
                    cursor: cursor.clone(),
                    limit: Some(1),
                },
                ref_: None,
            })
            .await
            .expect("Failed to list commits");

        assert!(
            response.commits.len() <= 1,
            "Expected at most 1 commit per page, got {}",
            response.commits.len()
        );

        for commit in &response.commits {
            collected_shas.push(commit.sha.clone());
        }

        pages += 1;

        if !response.has_more {
            assert!(
                response.next_cursor.is_none(),
                "next_cursor should be None on the last page"
            );
            break;
        }

        assert!(
            response.next_cursor.is_some(),
            "next_cursor should be Some when has_more is true"
        );
        cursor = response.next_cursor;
    }

    assert!(pages >= 3, "Expected at least 3 pages, got {pages}");

    for sha in &commit_shas {
        assert!(
            collected_shas.contains(sha),
            "Commit '{sha}' not found in paginated results"
        );
    }
}

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_list_all_commits_as_stream(ctx: &mut RepoWithCommitContext) {
    use futures::{StreamExt, pin_mut};

    let mut commit_shas: Vec<String> = vec![ctx.commit_sha.clone()];

    for i in 0..2 {
        let commit = ctx
            .client
            .commits(&ctx.org, &ctx.repo_name)
            .create(&CreateCommitRequest {
                branch: "main".to_owned(),
                message: format!("Stream commit {i}"),
                author: test_author(),
                files: vec![CommitFile {
                    action: CommitFileAction::Upsert,
                    path: format!("stream{i}.txt"),
                    content: Some(format!("content {i}")),
                }],
                base_sha: None,
            })
            .await
            .expect("Failed to create commit");
        commit_shas.push(commit.sha);
    }

    let stream = ctx.client.commits(&ctx.org, &ctx.repo_name).list_all(None);
    pin_mut!(stream);

    let mut found_shas: Vec<String> = Vec::new();
    while let Some(result) = stream.next().await {
        let commit = result.expect("Failed to get commit from stream");
        found_shas.push(commit.sha.clone());
    }

    for sha in &commit_shas {
        assert!(
            found_shas.contains(sha),
            "Commit '{sha}' not found in stream"
        );
    }
}

#[tokio::test]
async fn test_list_all_commits_with_ref() {
    let client = test_client();
    let org = test_org();
    let repo_name = unique_repo_name("cmref");

    client
        .repos(&org)
        .create(&CreateRepoRequest {
            name: repo_name.clone(),
            default_branch: None,
        })
        .await
        .expect("Failed to create repo");

    // Create initial commit on main
    let main_commit = client
        .commits(&org, &repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Main commit".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "main.txt".to_owned(),
                content: Some("main content".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("Failed to create main commit");

    // Create a feature branch
    client
        .branches(&org, &repo_name)
        .create(&CreateBranchRequest {
            name: "feature".to_owned(),
            from: "main".to_owned(),
        })
        .await
        .expect("Failed to create feature branch");

    // Add a commit only on the feature branch
    let feature_commit = client
        .commits(&org, &repo_name)
        .create(&CreateCommitRequest {
            branch: "feature".to_owned(),
            message: "Feature commit".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "feature.txt".to_owned(),
                content: Some("feature content".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("Failed to create feature commit");

    let result = AssertUnwindSafe(async {
        // List commits on main — should include the main commit but NOT the feature commit
        let main_commits = client
            .commits(&org, &repo_name)
            .list_all(Some("main"))
            .collect()
            .await
            .expect("Failed to list main commits");

        let main_shas: Vec<&str> = main_commits.iter().map(|c| c.sha.as_str()).collect();
        assert!(
            main_shas.contains(&main_commit.sha.as_str()),
            "Main commit should appear in main ref listing"
        );
        assert!(
            !main_shas.contains(&feature_commit.sha.as_str()),
            "Feature commit should NOT appear in main ref listing"
        );

        // List commits on feature — should include both (feature inherits main history)
        let feature_commits = client
            .commits(&org, &repo_name)
            .list_all(Some("feature"))
            .collect()
            .await
            .expect("Failed to list feature commits");

        let feature_shas: Vec<&str> = feature_commits.iter().map(|c| c.sha.as_str()).collect();
        assert!(
            feature_shas.contains(&feature_commit.sha.as_str()),
            "Feature commit should appear in feature ref listing"
        );
        assert!(
            feature_shas.contains(&main_commit.sha.as_str()),
            "Main commit should appear in feature ref listing (inherited history)"
        );
    })
    .catch_unwind()
    .await;

    let _ = client.repos(&org).delete(&repo_name).await;
    result.expect("test assertions failed");
}
