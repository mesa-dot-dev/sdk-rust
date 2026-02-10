#![allow(
    missing_docs,
    dead_code,
    unused_imports,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::doc_markdown
)]

#[path = "common/mod.rs"]
mod common;

// Admin
#[path = "low_level/admin/create_api_key.rs"]
mod create_api_key;
#[path = "low_level/admin/list_api_keys.rs"]
mod list_api_keys;
#[path = "low_level/admin/revoke_api_key.rs"]
mod revoke_api_key;

// Branches
#[path = "low_level/branches/create_branch.rs"]
mod create_branch;
#[path = "low_level/branches/delete_branch.rs"]
mod delete_branch;
#[path = "low_level/branches/list_branches.rs"]
mod list_branches;

// Commits
#[path = "low_level/commits/create_commit.rs"]
mod create_commit;
#[path = "low_level/commits/get_commit.rs"]
mod get_commit;
#[path = "low_level/commits/list_commits.rs"]
mod list_commits;

// Content
#[path = "low_level/content/get_content.rs"]
mod get_content;

// Diffs
#[path = "low_level/diffs/get_diff.rs"]
mod get_diff;

// Org
#[path = "low_level/org/get_organization.rs"]
mod get_organization;

// Repos
#[path = "low_level/repos/create_repository.rs"]
mod create_repository;
#[path = "low_level/repos/delete_repository.rs"]
mod delete_repository;
#[path = "low_level/repos/get_repository.rs"]
mod get_repository;
#[path = "low_level/repos/get_sync_status.rs"]
mod get_sync_status;
#[path = "low_level/repos/list_repositories.rs"]
mod list_repositories;
#[path = "low_level/repos/sync_repository.rs"]
mod sync_repository;
