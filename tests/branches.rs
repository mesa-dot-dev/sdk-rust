#![allow(missing_docs, dead_code)]

mod common;

use std::panic::AssertUnwindSafe;

use common::{RepoWithCommitContext, test_author, test_client, test_org, unique_repo_name};
use futures::FutureExt;
use mesa_dev::models::{
    CommitFile, CommitFileAction, CreateBranchRequest, CreateCommitRequest, CreateRepoRequest,
    PaginationParams,
};
use test_context::test_context;

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_create_and_list_branch(ctx: &mut RepoWithCommitContext) {
    let branch = ctx
        .client
        .branches(&ctx.org, &ctx.repo_name)
        .create(&CreateBranchRequest {
            name: "feature-test".to_owned(),
            from: "main".to_owned(),
        })
        .await
        .expect("Failed to create branch");

    assert_eq!(branch.name, "feature-test");
    assert!(!branch.is_default);

    let response = ctx
        .client
        .branches(&ctx.org, &ctx.repo_name)
        .list(&PaginationParams {
            cursor: None,
            limit: None,
        })
        .await
        .expect("Failed to list branches");

    let names: Vec<&str> = response.branches.iter().map(|b| b.name.as_str()).collect();
    assert!(names.contains(&"main"), "Should contain main branch");
    assert!(
        names.contains(&"feature-test"),
        "Should contain created branch"
    );
}

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_list_all_branches(ctx: &mut RepoWithCommitContext) {
    for i in 0..3 {
        ctx.client
            .branches(&ctx.org, &ctx.repo_name)
            .create(&CreateBranchRequest {
                name: format!("branch-{i}"),
                from: "main".to_owned(),
            })
            .await
            .expect("Failed to create branch");
    }

    let all = ctx
        .client
        .branches(&ctx.org, &ctx.repo_name)
        .list_all()
        .collect()
        .await
        .expect("Failed to list all branches");

    assert!(
        all.len() >= 4,
        "Expected at least 4 branches, got {}",
        all.len()
    );
}

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_delete_branch(ctx: &mut RepoWithCommitContext) {
    ctx.client
        .branches(&ctx.org, &ctx.repo_name)
        .create(&CreateBranchRequest {
            name: "to-delete".to_owned(),
            from: "main".to_owned(),
        })
        .await
        .expect("Failed to create branch");

    let result = ctx
        .client
        .branches(&ctx.org, &ctx.repo_name)
        .delete("to-delete")
        .await
        .expect("Failed to delete branch");

    assert!(result.success);

    let branches = ctx
        .client
        .branches(&ctx.org, &ctx.repo_name)
        .list(&PaginationParams {
            cursor: None,
            limit: None,
        })
        .await
        .expect("Failed to list branches");

    assert!(
        !branches.branches.iter().any(|b| b.name == "to-delete"),
        "Deleted branch should not appear in list"
    );
}

#[tokio::test]
async fn test_list_branches_raw_pagination() {
    let client = test_client();
    let org = test_org();
    let repo_name = unique_repo_name("brpage");

    client
        .repos(&org)
        .create(&CreateRepoRequest {
            name: repo_name.clone(),
            default_branch: None,
        })
        .await
        .expect("Failed to create repo");

    // Need an initial commit so we can create branches
    client
        .commits(&org, &repo_name)
        .create(&CreateCommitRequest {
            branch: "main".to_owned(),
            message: "Initial commit".to_owned(),
            author: test_author(),
            files: vec![CommitFile {
                action: CommitFileAction::Upsert,
                path: "README.md".to_owned(),
                content: Some("# Test".to_owned()),
            }],
            base_sha: None,
        })
        .await
        .expect("Failed to create initial commit");

    let branch_names: Vec<String> = (0..3).map(|i| format!("page-branch-{i}")).collect();

    for name in &branch_names {
        client
            .branches(&org, &repo_name)
            .create(&CreateBranchRequest {
                name: name.clone(),
                from: "main".to_owned(),
            })
            .await
            .expect("Failed to create branch");
    }

    let result = AssertUnwindSafe(async {
        let mut collected_names: Vec<String> = Vec::new();
        let mut cursor: Option<String> = None;
        let mut pages = 0;

        loop {
            let response = client
                .branches(&org, &repo_name)
                .list(&PaginationParams {
                    cursor: cursor.clone(),
                    limit: Some(1),
                })
                .await
                .expect("Failed to list branches");

            assert!(
                response.branches.len() <= 1,
                "Expected at most 1 branch per page, got {}",
                response.branches.len()
            );

            for branch in &response.branches {
                collected_names.push(branch.name.clone());
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

        // 3 created + main = at least 4 pages
        assert!(pages >= 4, "Expected at least 4 pages, got {pages}");

        for name in &branch_names {
            assert!(
                collected_names.contains(name),
                "Branch '{name}' not found in paginated results"
            );
        }
        assert!(
            collected_names.contains(&"main".to_owned()),
            "main branch not found in paginated results"
        );
    })
    .catch_unwind()
    .await;

    let _ = client.repos(&org).delete(&repo_name).await;
    result.expect("test assertions failed");
}

#[test_context(RepoWithCommitContext)]
#[tokio::test]
async fn test_list_all_branches_as_stream(ctx: &mut RepoWithCommitContext) {
    use futures::{StreamExt, pin_mut};

    let branch_names: Vec<String> = (0..3).map(|i| format!("stream-branch-{i}")).collect();

    for name in &branch_names {
        ctx.client
            .branches(&ctx.org, &ctx.repo_name)
            .create(&CreateBranchRequest {
                name: name.clone(),
                from: "main".to_owned(),
            })
            .await
            .expect("Failed to create branch");
    }

    let stream = ctx.client.branches(&ctx.org, &ctx.repo_name).list_all();
    pin_mut!(stream);

    let mut found_names: Vec<String> = Vec::new();
    while let Some(result) = stream.next().await {
        let branch = result.expect("Failed to get branch from stream");
        found_names.push(branch.name.clone());
    }

    for name in &branch_names {
        assert!(
            found_names.contains(name),
            "Branch '{name}' not found in stream"
        );
    }
    assert!(
        found_names.contains(&"main".to_owned()),
        "main branch not found in stream"
    );
}
