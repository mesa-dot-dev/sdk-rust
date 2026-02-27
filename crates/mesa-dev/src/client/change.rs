use std::sync::atomic::{AtomicU64, Ordering};

use tonic::service::interceptor::InterceptedService;
use tonic::transport::Channel;

use crate::grpc::{
    self, control_service_client::ControlServiceClient, data_service_client::DataServiceClient,
};

/// Interceptor that attaches a bearer token to every gRPC request.
#[derive(Clone, Debug)]
struct AuthInterceptor {
    token: tonic::metadata::MetadataValue<tonic::metadata::Ascii>,
}

impl tonic::service::Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut request: tonic::Request<()>,
    ) -> Result<tonic::Request<()>, tonic::Status> {
        request
            .metadata_mut()
            .insert("authorization", self.token.clone());
        Ok(request)
    }
}

/// Client for change-based file operations (`/{org}/{repo}/changes`).
///
/// Uses the gRPC `ControlService` to create changes, apply file operations
/// (create, modify, delete, move), and snapshot changes into commits.
///
/// # Example
///
/// ```rust,no_run
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// use mesa_dev::MesaClient;
///
/// let client = MesaClient::builder()
///     .build()?;
///
/// let org = client.org("my-org");
/// let repo = org.repos().at("my-repo");
/// let change_client = repo.change().await?;
///
/// // Create a change based on main branch
/// let change = change_client.create_from_ref("refs/heads/main").await?;
/// let change_id = change.id.unwrap();
///
/// // Apply file operations
/// change_client.create_file(
///     &change_id,
///     "hello.txt",
///     b"Hello, world!",
///     None,
/// ).await?;
///
/// // Snapshot to a commit
/// change_client.snapshot(&change_id, "Add hello.txt").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct ChangeClient {
    client: ControlServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    data_client: DataServiceClient<InterceptedService<Channel, AuthInterceptor>>,
    repo_id: String,
    /// Tracks the last applied op id for strict sequencing.
    /// Starts at 0 for a fresh change and is updated after each `apply_ops`.
    last_op_id: AtomicU64,
}

impl ChangeClient {
    /// Build a `ChangeClient` from a gRPC channel, bearer token, and repo UUID.
    pub(super) fn new(channel: &Channel, bearer_token: Option<&str>, repo_id: String) -> Self {
        let auth_value = bearer_token
            .map(|t| format!("Bearer {t}"))
            .unwrap_or_default();

        // MetadataValue::try_from can only fail on non-ASCII, which won't
        // happen for bearer tokens.
        let token = auth_value
            .try_into()
            .unwrap_or_else(|_| tonic::metadata::MetadataValue::from_static(""));

        let interceptor = AuthInterceptor { token };

        Self {
            client: ControlServiceClient::with_interceptor(channel.clone(), interceptor.clone()),
            data_client: DataServiceClient::with_interceptor(channel.clone(), interceptor),
            repo_id,
            last_op_id: AtomicU64::new(0),
        }
    }

    /// Create a new change based on a ref name (e.g. `"refs/heads/main"`).
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self), fields(repo_id = %self.repo_id), err(Debug))]
    pub async fn create_from_ref(
        &self,
        base_ref_name: &str,
    ) -> Result<grpc::Change, tonic::Status> {
        let resp = self
            .client
            .clone()
            .create_change(grpc::CreateChangeRequest {
                repo_id: self.repo_id.clone(),
                base: Some(grpc::create_change_request::Base::BaseRefName(
                    base_ref_name.to_owned(),
                )),
            })
            .await?;

        resp.into_inner()
            .change
            .ok_or_else(|| tonic::Status::internal("server returned empty change"))
    }

    /// Create a new change based on a commit OID.
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self, commit_oid), fields(repo_id = %self.repo_id), err(Debug))]
    pub async fn create_from_commit(
        &self,
        commit_oid: &[u8],
    ) -> Result<grpc::Change, tonic::Status> {
        let resp = self
            .client
            .clone()
            .create_change(grpc::CreateChangeRequest {
                repo_id: self.repo_id.clone(),
                base: Some(grpc::create_change_request::Base::BaseCommit(
                    grpc::CommitOid {
                        value: commit_oid.to_vec(),
                    },
                )),
            })
            .await?;

        resp.into_inner()
            .change
            .ok_or_else(|| tonic::Status::internal("server returned empty change"))
    }

    /// Create a file in the working copy of a change.
    ///
    /// Uses inline data for the file content. For large files, use
    /// [`apply_ops`](Self::apply_ops) with an `OpCreateFile` referencing a pre-uploaded blob OID.
    ///
    /// `mode` defaults to `0o100644` (regular file) if `None`.
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self, content), fields(repo_id = %self.repo_id, path), err(Debug))]
    pub async fn create_file(
        &self,
        change_id: &grpc::ChangeId,
        path: &str,
        content: &[u8],
        mode: Option<u32>,
    ) -> Result<grpc::ApplyOpsResponse, tonic::Status> {
        let op = grpc::Op {
            op: Some(grpc::op::Op::CreateFile(grpc::OpCreateFile {
                path: path.to_owned(),
                mode: mode.unwrap_or(0o100_644),
                content: Some(grpc::op_create_file::Content::InlineData(content.to_vec())),
            })),
        };
        self.apply_ops(change_id, vec![op]).await
    }

    /// Modify an existing file in the working copy of a change.
    ///
    /// Replaces the file content with `content` (inline data).
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self, content), fields(repo_id = %self.repo_id, path), err(Debug))]
    pub async fn modify_file(
        &self,
        change_id: &grpc::ChangeId,
        path: &str,
        content: &[u8],
    ) -> Result<grpc::ApplyOpsResponse, tonic::Status> {
        let op = grpc::Op {
            op: Some(grpc::op::Op::ModifyFile(grpc::OpModifyFile {
                path: path.to_owned(),
                content: Some(grpc::op_modify_file::Content::InlineData(content.to_vec())),
            })),
        };
        self.apply_ops(change_id, vec![op]).await
    }

    /// Delete a path from the working copy of a change.
    ///
    /// Set `recursive` to `true` to delete a directory and all its contents.
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self), fields(repo_id = %self.repo_id, path), err(Debug))]
    pub async fn delete_path(
        &self,
        change_id: &grpc::ChangeId,
        path: &str,
        recursive: bool,
    ) -> Result<grpc::ApplyOpsResponse, tonic::Status> {
        let op = grpc::Op {
            op: Some(grpc::op::Op::DeletePath(grpc::OpDeletePath {
                path: path.to_owned(),
                recursive,
            })),
        };
        self.apply_ops(change_id, vec![op]).await
    }

    /// Move (rename) a path in the working copy of a change.
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self), fields(repo_id = %self.repo_id, from_path, to_path), err(Debug))]
    pub async fn move_path(
        &self,
        change_id: &grpc::ChangeId,
        from_path: &str,
        to_path: &str,
    ) -> Result<grpc::ApplyOpsResponse, tonic::Status> {
        let op = grpc::Op {
            op: Some(grpc::op::Op::MovePath(grpc::OpMovePath {
                from_path: from_path.to_owned(),
                to_path: to_path.to_owned(),
            })),
        };
        self.apply_ops(change_id, vec![op]).await
    }

    /// Apply a batch of operations to a change in a single RPC.
    ///
    /// This is the low-level method underlying [`create_file`](Self::create_file),
    /// [`modify_file`](Self::modify_file), [`delete_path`](Self::delete_path), and
    /// [`move_path`](Self::move_path). Use it when you need to apply multiple
    /// operations atomically.
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self, ops), fields(repo_id = %self.repo_id, op_count = ops.len()), err(Debug))]
    pub async fn apply_ops(
        &self,
        change_id: &grpc::ChangeId,
        ops: Vec<grpc::Op>,
    ) -> Result<grpc::ApplyOpsResponse, tonic::Status> {
        let request = grpc::ApplyOpsRequest {
            repo_id: self.repo_id.clone(),
            change_id: Some(change_id.clone()),
            expected_last_op_id: Some(self.last_op_id.load(Ordering::Relaxed)),
            idempotency_key: String::new(),
            ops,
        };

        let resp = self
            .client
            .clone()
            .apply_ops(tokio_stream::once(request))
            .await?;

        let inner = resp.into_inner();
        self.last_op_id.store(inner.last_op_id, Ordering::Relaxed);
        Ok(inner)
    }

    /// Snapshot the current working-copy state of a change into a commit.
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self), fields(repo_id = %self.repo_id), err(Debug))]
    pub async fn snapshot(
        &self,
        change_id: &grpc::ChangeId,
        message: &str,
    ) -> Result<grpc::SnapshotChangeResponse, tonic::Status> {
        let resp = self
            .client
            .clone()
            .snapshot_change(grpc::SnapshotChangeRequest {
                repo_id: self.repo_id.clone(),
                change_id: Some(change_id.clone()),
                expected_last_op_id: Some(self.last_op_id.load(Ordering::Relaxed)),
                message: message.to_owned(),
                idempotency_key: String::new(),
            })
            .await?;

        Ok(resp.into_inner())
    }

    /// Move (update) a bookmark to point at a new commit OID.
    ///
    /// Use this after [`snapshot`](Self::snapshot) to make the new commit visible
    /// via the REST content API by updating the ref the bookmark maps to.
    ///
    /// `expected_update_seq` is the optimistic concurrency token from a prior
    /// bookmark response (use `0` if unknown / first update).
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self, commit_oid), fields(repo_id = %self.repo_id, bookmark_name), err(Debug))]
    pub async fn move_bookmark(
        &self,
        bookmark_name: &str,
        commit_oid: &[u8],
        expected_update_seq: u64,
    ) -> Result<grpc::MoveBookmarkResponse, tonic::Status> {
        let resp = self
            .client
            .clone()
            .move_bookmark(grpc::MoveBookmarkRequest {
                repo_id: self.repo_id.clone(),
                bookmark_name: bookmark_name.to_owned(),
                expected_update_seq,
                new_commit_oid: Some(grpc::CommitOid {
                    value: commit_oid.to_vec(),
                }),
            })
            .await?;

        Ok(resp.into_inner())
    }

    /// Resolve a ref name to its current target OID and metadata.
    ///
    /// Returns a [`RefInfo`](grpc::RefInfo) containing the ref's `update_seq`,
    /// which is needed as the optimistic-concurrency token for
    /// [`move_bookmark`](Self::move_bookmark).
    ///
    /// # Errors
    ///
    /// Returns a gRPC error if the request fails.
    #[tracing::instrument(skip(self), fields(repo_id = %self.repo_id, ref_name), err(Debug))]
    pub async fn resolve_ref(&self, ref_name: &str) -> Result<grpc::RefInfo, tonic::Status> {
        let resp = self
            .data_client
            .clone()
            .resolve_ref(grpc::ResolveRefRequest {
                repo_id: self.repo_id.clone(),
                ref_name: ref_name.to_owned(),
            })
            .await?;

        resp.into_inner()
            .r#ref
            .ok_or_else(|| tonic::Status::internal("server returned empty ref"))
    }
}
