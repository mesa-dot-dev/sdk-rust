#![allow(missing_docs, dead_code)]

mod common;

use common::{test_author, test_client, test_org, unique_repo_name};
use mesa_dev::models::{
    CommitFile, CommitFileAction, Content, CreateCommitRequest, CreateRepoRequest,
};
use test_context::AsyncTestContext;
use test_context::test_context;

/// Content-specific context: creates a repo with 3 files (README.md, src/main.rs, src/lib.rs).
struct ContentRepoContext {
    client: mesa_dev::Mesa,
    org: String,
    repo_name: String,
}

impl AsyncTestContext for ContentRepoContext {
    async fn setup() -> Self {
        let client = test_client();
        let org = test_org();
        let repo_name = unique_repo_name("content");

        client
            .repos(&org)
            .create(&CreateRepoRequest {
                name: repo_name.clone(),
                default_branch: None,
            })
            .await
            .expect("ContentRepoContext: failed to create repo");

        client
            .commits(&org, &repo_name)
            .create(&CreateCommitRequest {
                branch: "main".to_owned(),
                message: "Add test files".to_owned(),
                author: test_author(),
                files: vec![
                    CommitFile {
                        action: CommitFileAction::Upsert,
                        path: "README.md".to_owned(),
                        content: Some("# Hello World".to_owned()),
                    },
                    CommitFile {
                        action: CommitFileAction::Upsert,
                        path: "src/main.rs".to_owned(),
                        content: Some("fn main() {}".to_owned()),
                    },
                    CommitFile {
                        action: CommitFileAction::Upsert,
                        path: "src/lib.rs".to_owned(),
                        content: Some("pub fn add(a: i32, b: i32) -> i32 { a + b }".to_owned()),
                    },
                ],
                base_sha: None,
            })
            .await
            .expect("ContentRepoContext: failed to create commit");

        Self {
            client,
            org,
            repo_name,
        }
    }

    async fn teardown(self) {
        let _ = self.client.repos(&self.org).delete(&self.repo_name).await;
    }
}

#[test_context(ContentRepoContext)]
#[tokio::test]
async fn test_get_file_content(ctx: &mut ContentRepoContext) {
    let content = ctx
        .client
        .content(&ctx.org, &ctx.repo_name)
        .get(Some("README.md"), None)
        .await
        .expect("get file content");

    match content {
        Content::File {
            name,
            path,
            content,
            ..
        } => {
            assert_eq!(name, "README.md");
            assert_eq!(path, "README.md");
            assert!(!content.is_empty());
        }
        Content::Dir { .. } => panic!("Expected file, got directory"),
    }
}

#[test_context(ContentRepoContext)]
#[tokio::test]
async fn test_get_directory_content(ctx: &mut ContentRepoContext) {
    let content = ctx
        .client
        .content(&ctx.org, &ctx.repo_name)
        .get(Some("src"), None)
        .await
        .expect("get dir content");

    match content {
        Content::Dir { name, entries, .. } => {
            assert_eq!(name, "src");
            assert!(entries.len() >= 2, "Expected at least 2 entries in src/");
            let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
            assert!(names.contains(&"main.rs"));
            assert!(names.contains(&"lib.rs"));
        }
        Content::File { .. } => panic!("Expected directory, got file"),
    }
}

#[test_context(ContentRepoContext)]
#[tokio::test]
async fn test_get_root_content(ctx: &mut ContentRepoContext) {
    let content = ctx
        .client
        .content(&ctx.org, &ctx.repo_name)
        .get(None, None)
        .await
        .expect("get root content");

    match content {
        Content::Dir { entries, .. } => {
            let names: Vec<&str> = entries.iter().map(|e| e.name.as_str()).collect();
            assert!(
                names.contains(&"README.md"),
                "Root should contain README.md"
            );
            assert!(names.contains(&"src"), "Root should contain src/");
        }
        Content::File { .. } => panic!("Expected directory listing for root"),
    }
}

#[test_context(ContentRepoContext)]
#[tokio::test]
async fn test_get_content_at_ref(ctx: &mut ContentRepoContext) {
    let content = ctx
        .client
        .content(&ctx.org, &ctx.repo_name)
        .get(Some("README.md"), Some("main"))
        .await
        .expect("get content at ref");

    match content {
        Content::File { name, .. } => {
            assert_eq!(name, "README.md");
        }
        Content::Dir { .. } => panic!("Expected file"),
    }
}
