#![allow(missing_docs, dead_code)]

mod common;

use std::panic::AssertUnwindSafe;

use common::{RepoContext, test_client, test_org, unique_repo_name};
use futures::FutureExt;
use mesa_dev::models::{CreateRepoRequest, PaginationParams, RenameRepoRequest};
use test_context::test_context;

#[test_context(RepoContext)]
#[tokio::test]
async fn test_create_and_get_repo(ctx: &mut RepoContext) {
    // Repo already created by context — verify it exists
    let fetched = ctx
        .client
        .repos(&ctx.org)
        .get(&ctx.repo_name)
        .await
        .expect("Failed to get repo");

    assert_eq!(fetched.name, ctx.repo_name);
    assert_eq!(fetched.default_branch, "main");
}

#[tokio::test]
async fn test_create_repo_with_custom_default_branch() {
    let client = test_client();
    let org = test_org();
    let repo_name = unique_repo_name("branch");

    client
        .repos(&org)
        .create(&CreateRepoRequest {
            name: repo_name.clone(),
            default_branch: Some("develop".to_owned()),
        })
        .await
        .expect("Failed to create repo");

    let result = AssertUnwindSafe(async {
        let fetched = client
            .repos(&org)
            .get(&repo_name)
            .await
            .expect("Failed to get repo");

        assert_eq!(fetched.default_branch, "develop");
    })
    .catch_unwind()
    .await;

    let _ = client.repos(&org).delete(&repo_name).await;
    result.expect("test assertions failed");
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_list_repos(ctx: &mut RepoContext) {
    let response = ctx
        .client
        .repos(&ctx.org)
        .list(&PaginationParams {
            cursor: None,
            limit: Some(10),
        })
        .await
        .expect("Failed to list repos");

    assert!(!response.repos.is_empty(), "Expected at least one repo");
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_list_all_repos(ctx: &mut RepoContext) {
    let all_repos = ctx
        .client
        .repos(&ctx.org)
        .list_all()
        .collect()
        .await
        .expect("Failed to list all repos");

    assert!(
        all_repos.iter().any(|r| r.name == ctx.repo_name),
        "Created repo should appear in list_all"
    );
}

#[test_context(RepoContext)]
#[tokio::test]
async fn test_list_all_repos_as_stream(ctx: &mut RepoContext) {
    use futures::{StreamExt, pin_mut};

    let stream = ctx.client.repos(&ctx.org).list_all();
    pin_mut!(stream);

    let mut found = false;
    while let Some(result) = stream.next().await {
        let repo = result.expect("Failed to get repo from stream");
        if repo.name == ctx.repo_name {
            found = true;
        }
    }

    assert!(found, "Created repo should appear in stream");
}

#[tokio::test]
async fn test_rename_repo() {
    let client = test_client();
    let org = test_org();
    let repo_name = unique_repo_name("rename");
    let new_name = unique_repo_name("renamed");

    client
        .repos(&org)
        .create(&CreateRepoRequest {
            name: repo_name.clone(),
            default_branch: None,
        })
        .await
        .expect("Failed to create repo");

    let result = AssertUnwindSafe(async {
        let renamed = client
            .repos(&org)
            .rename(
                &repo_name,
                &RenameRepoRequest {
                    name: new_name.clone(),
                },
            )
            .await
            .expect("Failed to rename repo");

        assert_eq!(renamed.name, new_name);

        let err = client.repos(&org).get(&repo_name).await;
        assert!(err.is_err(), "Old repo name should not exist");
    })
    .catch_unwind()
    .await;

    // Clean up using the new name (rename may or may not have succeeded)
    let _ = client.repos(&org).delete(&new_name).await;
    let _ = client.repos(&org).delete(&repo_name).await;
    result.expect("test assertions failed");
}

#[tokio::test]
async fn test_delete_repo() {
    let client = test_client();
    let org = test_org();
    let repo_name = unique_repo_name("delete");

    client
        .repos(&org)
        .create(&CreateRepoRequest {
            name: repo_name.clone(),
            default_branch: None,
        })
        .await
        .expect("Failed to create repo");

    let result = client
        .repos(&org)
        .delete(&repo_name)
        .await
        .expect("Failed to delete repo");

    assert!(result.success);

    let err = client.repos(&org).get(&repo_name).await;
    assert!(err.is_err(), "Deleted repo should not exist");
}

#[tokio::test]
async fn test_list_repos_raw_pagination() {
    let client = test_client();
    let org = test_org();

    let repo_names: Vec<String> = (0..3)
        .map(|i| unique_repo_name(&format!("page{i}")))
        .collect();

    for name in &repo_names {
        client
            .repos(&org)
            .create(&CreateRepoRequest {
                name: name.clone(),
                default_branch: None,
            })
            .await
            .expect("Failed to create repo");
    }

    let result = AssertUnwindSafe(async {
        let mut collected_names: Vec<String> = Vec::new();
        let mut cursor: Option<String> = None;
        let mut pages = 0;

        loop {
            let response = client
                .repos(&org)
                .list(&PaginationParams {
                    cursor: cursor.clone(),
                    limit: Some(1),
                })
                .await
                .expect("Failed to list repos");

            assert!(
                response.repos.len() <= 1,
                "Expected at most 1 repo per page, got {}",
                response.repos.len()
            );

            for repo in &response.repos {
                collected_names.push(repo.name.clone());
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

        for name in &repo_names {
            assert!(
                collected_names.contains(name),
                "Repo '{name}' not found in paginated results"
            );
        }
    })
    .catch_unwind()
    .await;

    for name in &repo_names {
        let _ = client.repos(&org).delete(name).await;
    }
    result.expect("test assertions failed");
}

#[tokio::test]
async fn test_list_all_repos_next_page() {
    let client = test_client();
    let org = test_org();

    let repo_names: Vec<String> = (0..3)
        .map(|i| unique_repo_name(&format!("np{i}")))
        .collect();

    for name in &repo_names {
        client
            .repos(&org)
            .create(&CreateRepoRequest {
                name: name.clone(),
                default_branch: None,
            })
            .await
            .expect("Failed to create repo");
    }

    let result = AssertUnwindSafe(async {
        let mut stream = client.repos(&org).list_all();
        let mut all_names: Vec<String> = Vec::new();
        let mut pages = 0;

        while let Some(page) = stream.next_page().await.expect("Failed to get next page") {
            pages += 1;

            assert!(
                !page.repos.is_empty(),
                "Each page should have at least one repo"
            );

            for repo in &page.repos {
                all_names.push(repo.name.clone());
            }

            // On the last page, has_more should be false
            if !page.has_more {
                assert!(
                    page.next_cursor.is_none(),
                    "next_cursor should be None on the last page"
                );
            }
        }

        assert!(pages >= 1, "Expected at least 1 page, got {pages}");

        for name in &repo_names {
            assert!(
                all_names.contains(name),
                "Repo '{name}' not found in next_page results"
            );
        }
    })
    .catch_unwind()
    .await;

    for name in &repo_names {
        let _ = client.repos(&org).delete(name).await;
    }
    result.expect("test assertions failed");
}
