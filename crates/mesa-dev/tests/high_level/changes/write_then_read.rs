use crate::common::HlRepoWithCommitContext;
use mesa_dev::low_level::content::Content;
use test_context::test_context;

/// Write a file via gRPC (change API), snapshot it to a commit, move the
/// bookmark so the commit is visible via REST, then read it back via the
/// REST content API and verify the content matches.
#[test_context(HlRepoWithCommitContext)]
#[tokio::test]
async fn test_write_then_read(ctx: &mut HlRepoWithCommitContext) {
    let org = ctx.client.org(&ctx.org);
    let repo = org.repos().at(&ctx.repo_name);
    let change_client = repo.change().await.unwrap();

    // 1. Create a change based on main
    let change = change_client
        .create_from_ref("refs/heads/main")
        .await
        .unwrap();
    let change_id = change.id.unwrap();

    // 2. Write a new file via gRPC
    let file_content = b"Hello from the write-then-read test!";
    let resp = change_client
        .create_file(&change_id, "test-file.txt", file_content, None)
        .await
        .unwrap();
    assert!(
        resp.applied_ops_count > 0,
        "should have applied at least one op"
    );

    // 3. Snapshot to commit the change
    let snapshot = change_client
        .snapshot(&change_id, "Add test-file.txt")
        .await
        .unwrap();
    let commit_oid = snapshot
        .commit_oid
        .expect("snapshot should produce a commit");
    let commit_hex = hex::encode(&commit_oid.value);

    // 4. Flush: move the "main" bookmark to the new commit so the REST
    //    content API can resolve it.
    let ref_info = change_client
        .resolve_ref("refs/heads/main")
        .await
        .expect("resolve_ref should succeed");
    change_client
        .move_bookmark("main", &commit_oid.value, ref_info.update_seq)
        .await
        .expect("move_bookmark should succeed");

    // 5. Read the file back via REST content API, pinned to the new commit
    let content = repo
        .content()
        .get(Some(&commit_hex), Some("test-file.txt"), None)
        .await
        .expect("get content should succeed for the newly written file");

    // 6. Verify the content matches
    match content {
        Content::File(file) => {
            assert_eq!(file.name.as_deref(), Some("test-file.txt"));
            assert_eq!(file.path.as_deref(), Some("test-file.txt"));

            let encoded = file.content.expect("file content should be present");
            let decoded = base64_decode(&encoded);
            assert_eq!(
                decoded, file_content,
                "file content read from REST should match what was written via gRPC"
            );
        }
        other => panic!("expected Content::File, got {other:?}"),
    }
}

/// Decode base64 content, stripping any whitespace (the API may return
/// multi-line base64).
fn base64_decode(encoded: &str) -> Vec<u8> {
    use base64::Engine;
    let cleaned: String = encoded.chars().filter(|c| !c.is_whitespace()).collect();
    base64::engine::general_purpose::STANDARD
        .decode(&cleaned)
        .expect("file content should be valid base64")
}
