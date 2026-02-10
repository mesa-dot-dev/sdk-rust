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

// Org
#[path = "high_level/org/get_organization.rs"]
mod hl_get_organization;

// Repos
#[path = "high_level/repos/create_repository.rs"]
mod hl_create_repository;
#[path = "high_level/repos/delete_repository.rs"]
mod hl_delete_repository;
#[path = "high_level/repos/get_repository.rs"]
mod hl_get_repository;
#[path = "high_level/repos/list_repositories.rs"]
mod hl_list_repositories;
#[path = "high_level/repos/update_repository.rs"]
mod hl_update_repository;

// Branches
#[path = "high_level/branches/create_branch.rs"]
mod hl_create_branch;
#[path = "high_level/branches/delete_branch.rs"]
mod hl_delete_branch;
#[path = "high_level/branches/list_branches.rs"]
mod hl_list_branches;

// Commits
#[path = "high_level/commits/create_commit.rs"]
mod hl_create_commit;
#[path = "high_level/commits/get_commit.rs"]
mod hl_get_commit;
#[path = "high_level/commits/list_commits.rs"]
mod hl_list_commits;

// Content
#[path = "high_level/content/get_content.rs"]
mod hl_get_content;

// Diffs
#[path = "high_level/diffs/get_diff.rs"]
mod hl_get_diff;

// Sync
#[path = "high_level/sync/get_sync_status.rs"]
mod hl_get_sync_status;

// Webhooks
#[path = "high_level/webhooks/create_webhook.rs"]
mod hl_create_webhook;
#[path = "high_level/webhooks/delete_webhook.rs"]
mod hl_delete_webhook;
#[path = "high_level/webhooks/list_webhooks.rs"]
mod hl_list_webhooks;

// Admin / API Keys
#[path = "high_level/admin/create_api_key.rs"]
mod hl_create_api_key;
#[path = "high_level/admin/list_api_keys.rs"]
mod hl_list_api_keys;
#[path = "high_level/admin/revoke_api_key.rs"]
mod hl_revoke_api_key;

// Analytics
#[path = "high_level/analytics/get_analytics.rs"]
mod hl_get_analytics;
#[path = "high_level/analytics/refresh_analytics.rs"]
mod hl_refresh_analytics;
