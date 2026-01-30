#![allow(missing_docs, dead_code)]

mod common;

use common::ApiKeyContext;
use mesa_dev::models::ApiKeyScope;
use test_context::test_context;

#[test_context(ApiKeyContext)]
#[tokio::test]
async fn test_create_and_list_api_keys(ctx: &mut ApiKeyContext) {
    // Key already created by context — verify it
    assert!(!ctx.created.id.is_empty());
    assert!(
        !ctx.created.key.is_empty(),
        "Should return the raw key on creation"
    );
    assert_eq!(ctx.created.name, Some("test-key-e2e".to_owned()));
    assert!(ctx.created.scopes.contains(&ApiKeyScope::GitRead));
    assert!(ctx.created.scopes.contains(&ApiKeyScope::RepoRead));

    let keys = ctx
        .client
        .admin(&ctx.org)
        .list_api_keys()
        .await
        .expect("list api keys");

    let found = keys.iter().find(|k| k.id == ctx.created.id);
    assert!(found.is_some(), "Created key should appear in list");
    assert_eq!(found.unwrap().name, Some("test-key-e2e".to_owned()));
}

#[test_context(ApiKeyContext)]
#[tokio::test]
async fn test_revoke_api_key(ctx: &mut ApiKeyContext) {
    // Revoke the key created by context
    let result = ctx
        .client
        .admin(&ctx.org)
        .revoke_api_key(&ctx.created.id)
        .await
        .expect("revoke api key");

    assert!(result.success);

    let keys = ctx
        .client
        .admin(&ctx.org)
        .list_api_keys()
        .await
        .expect("list api keys");

    let found = keys.iter().find(|k| k.id == ctx.created.id);
    assert!(found.is_some(), "Revoked key should still appear in list");
    assert!(
        found.unwrap().revoked_at.is_some(),
        "Revoked key should have revoked_at timestamp"
    );
}
