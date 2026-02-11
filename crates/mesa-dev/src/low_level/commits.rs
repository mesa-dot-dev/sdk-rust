//! Commits API with correct `anyOf` union handling for file operations.
//!
//! The `OpenAPI` code generator flattens `anyOf` request variants into a single
//! struct with every field required, which produces an incorrect request body
//! ([openapi-generator#9497](https://github.com/OpenAPITools/openapi-generator/issues/9497)).
//!
//! This module provides a hand-written [`CommitFile`] enum that correctly
//! serializes each file operation variant (upsert, delete, LFS) and a
//! [`create_commit`] wrapper that builds the request manually.
//!
//! The GET endpoints (list / get commit) in
//! [`crate::low_level::apis::commits_api`] work correctly and are not wrapped here.

use mesa_dev_oapi::apis::configuration::Configuration;
use mesa_dev_oapi::apis::{Error, ResponseContent};
use serde::{Deserialize, Serialize};

/// Typed errors returned by [`create_commit`].
pub use mesa_dev_oapi::apis::commits_api::PostByOrgByRepoCommitsError as CreateCommitError;

/// Encoding for upsert file content.
pub use mesa_dev_oapi::models::post_by_org_by_repo_commits_request_files_inner_any_of::Encoding;

/// Commit author / committer identity.
///
/// This is a re-export of the generated type with a friendlier name.
pub type CommitAuthor =
    mesa_dev_oapi::models::GetByOrgByRepoCommits200ResponseCommitsInnerCommitter;

/// A file operation within a commit.
///
/// Each variant maps to one of the `anyOf` shapes in the API spec.
#[derive(Clone, Debug, PartialEq)]
pub enum CommitFile {
    /// Create or update a file.
    Upsert {
        /// Path relative to the repository root.
        path: String,
        /// File content (text or base64-encoded).
        content: String,
        /// Content encoding. Defaults to UTF-8 when `None`.
        encoding: Option<Encoding>,
    },
    /// Delete a file.
    Delete {
        /// Path relative to the repository root.
        path: String,
    },
    /// Add an LFS pointer.
    Lfs {
        /// Path relative to the repository root.
        path: String,
        /// LFS object ID (SHA-256).
        oid: String,
        /// LFS object size in bytes.
        size: i64,
    },
}

impl Serialize for CommitFile {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Upsert {
                path,
                content,
                encoding,
            } => {
                #[derive(Serialize)]
                struct Wire<'a> {
                    action: &'a str,
                    path: &'a str,
                    content: &'a str,
                    #[serde(skip_serializing_if = "Option::is_none")]
                    encoding: &'a Option<Encoding>,
                }
                Wire {
                    action: "upsert",
                    path,
                    content,
                    encoding,
                }
                .serialize(serializer)
            }
            Self::Delete { path } => {
                #[derive(Serialize)]
                struct Wire<'a> {
                    action: &'a str,
                    path: &'a str,
                }
                Wire {
                    action: "delete",
                    path,
                }
                .serialize(serializer)
            }
            Self::Lfs { path, oid, size } => {
                #[derive(Serialize)]
                struct Lfs<'a> {
                    oid: &'a str,
                    size: i64,
                }
                #[derive(Serialize)]
                struct Wire<'a> {
                    path: &'a str,
                    lfs: Lfs<'a>,
                }
                Wire {
                    path,
                    lfs: Lfs { oid, size: *size },
                }
                .serialize(serializer)
            }
        }
    }
}

/// Successful response from [`create_commit`].
///
/// The generated `PostByOrgByRepoCommits201Response` incorrectly marks all
/// fields as `Option<String>`; the API always returns them.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommitResponse {
    /// The SHA of the newly created commit.
    pub sha: String,
    /// The branch the commit was created on.
    pub branch: String,
    /// The commit message.
    pub message: String,
}

/// Programmatically create a commit with file operations.
///
/// This wraps the generated `post_by_org_by_repo_commits` endpoint with
/// correct `anyOf` request serialization. Each [`CommitFile`] variant is
/// serialized into the shape the API expects.
///
/// # Errors
///
/// Returns an error if the request fails or the response cannot be
/// deserialized.
#[allow(clippy::too_many_arguments)]
#[tracing::instrument(skip(configuration, author, files), fields(http.method = "POST", http.status_code), err(Debug))]
pub async fn create_commit(
    configuration: &Configuration,
    org: &str,
    repo: &str,
    branch: &str,
    message: &str,
    author: &CommitAuthor,
    files: &[CommitFile],
    base_sha: Option<&str>,
) -> Result<CommitResponse, Error<CreateCommitError>> {
    let mut body = serde_json::json!({
        "branch": branch,
        "message": message,
        "author": {
            "name": author.name,
            "email": author.email,
        },
        "files": files,
    });

    if let Some(sha) = base_sha {
        body.as_object_mut()
            .map(|m| m.insert("base_sha".to_string(), serde_json::json!(sha)));
    }

    let uri_str = format!(
        "{}/{org}/{repo}/commits",
        configuration.base_path,
        org = mesa_dev_oapi::apis::urlencode(org),
        repo = mesa_dev_oapi::apis::urlencode(repo),
    );
    let mut req_builder = configuration
        .client
        .request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    }
    req_builder = req_builder.json(&body);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    tracing::Span::current().record("http.status_code", status.as_u16());

    if !status.is_client_error() && !status.is_server_error() {
        let text = resp.text().await?;
        serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&text))
            .map_err(Error::from)
    } else {
        let text = resp.text().await?;
        let entity: Option<CreateCommitError> = serde_json::from_str(&text).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content: text,
            entity,
        }))
    }
}
