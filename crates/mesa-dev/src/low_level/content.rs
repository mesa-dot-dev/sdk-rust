//! Content API with correct `anyOf` union handling.
//!
//! The `OpenAPI` code generator flattens `anyOf` responses into a single struct
//! with every field required, which fails at runtime
//! ([openapi-generator#9497](https://github.com/OpenAPITools/openapi-generator/issues/9497)).
//!
//! This module provides a hand-written [`Content`] enum that correctly
//! discriminates on the `"type"` field and deserializes into the appropriate
//! variant.

use mesa_dev_oapi::apis::configuration::Configuration;
use mesa_dev_oapi::apis::{Error, ResponseContent};
use mesa_dev_oapi::models;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize};

/// Typed errors returned by [`get_content`].
pub use mesa_dev_oapi::apis::content_api::GetByOrgByRepoContentError as GetContentError;

/// Content returned by the content endpoint.
///
/// The API returns one of three shapes depending on whether the path points to
/// a file, a symbolic link, or a directory.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum Content {
    /// A regular file.
    File(models::GetByOrgByRepoContent200ResponseAnyOf),
    /// A symbolic link.
    Symlink(models::GetByOrgByRepoContent200ResponseAnyOf1),
    /// A directory listing.
    Dir(DirContent),
}

impl<'de> Deserialize<'de> for Content {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let type_str = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| D::Error::missing_field("type"))?;

        match type_str {
            "file" => serde_json::from_value(value)
                .map(Content::File)
                .map_err(D::Error::custom),
            "symlink" => serde_json::from_value(value)
                .map(Content::Symlink)
                .map_err(D::Error::custom),
            "dir" => serde_json::from_value(value)
                .map(Content::Dir)
                .map_err(D::Error::custom),
            other => Err(D::Error::unknown_variant(
                other,
                &["file", "symlink", "dir"],
            )),
        }
    }
}

/// A directory listing with correctly typed entries.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DirContent {
    /// Always `"dir"`.
    #[serde(rename = "type")]
    pub r#type: models::get_by_org_by_repo_content_200_response_any_of_2::Type,
    /// Directory name, or `None` for the repository root.
    #[serde(rename = "name", deserialize_with = "Option::deserialize")]
    pub name: Option<String>,
    /// Directory path relative to the repository root.
    #[serde(rename = "path", deserialize_with = "Option::deserialize")]
    pub path: Option<String>,
    /// Tree SHA.
    #[serde(rename = "sha", deserialize_with = "Option::deserialize")]
    pub sha: Option<String>,
    /// Total number of children in this directory.
    pub child_count: i64,
    /// Directory entries (files, symlinks, or subdirectories).
    pub entries: Vec<DirEntry>,
    /// Cursor for paginating entries, or `None` when there are no more pages.
    #[serde(rename = "next_cursor", deserialize_with = "Option::deserialize")]
    pub next_cursor: Option<String>,
    /// Whether more entries are available beyond this page.
    pub has_more: bool,
}

/// A single entry inside a directory listing.
#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum DirEntry {
    /// A file entry.
    File(models::GetByOrgByRepoContent200ResponseAnyOf2EntriesInnerAnyOf),
    /// A symbolic-link entry.
    Symlink(models::GetByOrgByRepoContent200ResponseAnyOf2EntriesInnerAnyOf1),
    /// A subdirectory entry.
    Dir(models::GetByOrgByRepoContent200ResponseAnyOf2EntriesInnerAnyOf2),
}

impl<'de> Deserialize<'de> for DirEntry {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(deserializer)?;
        let type_str = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| D::Error::missing_field("type"))?;

        match type_str {
            "file" => serde_json::from_value(value)
                .map(DirEntry::File)
                .map_err(D::Error::custom),
            "symlink" => serde_json::from_value(value)
                .map(DirEntry::Symlink)
                .map_err(D::Error::custom),
            "dir" => serde_json::from_value(value)
                .map(DirEntry::Dir)
                .map_err(D::Error::custom),
            other => Err(D::Error::unknown_variant(
                other,
                &["file", "symlink", "dir"],
            )),
        }
    }
}

/// Get file content or directory listing at a path.
///
/// This wraps the generated content API endpoint with correct `anyOf`
/// deserialization. Returns a [`Content`] enum discriminated on the `"type"`
/// field.
///
/// # Errors
///
/// Returns an error if the request fails or the response cannot be
/// deserialized.
pub async fn get_content(
    configuration: &Configuration,
    org: &str,
    repo: &str,
    r#ref: Option<&str>,
    path: Option<&str>,
    depth: Option<u64>,
) -> Result<Content, Error<GetContentError>> {
    let uri_str = format!(
        "{}/{org}/{repo}/content",
        configuration.base_path,
        org = mesa_dev_oapi::apis::urlencode(org),
        repo = mesa_dev_oapi::apis::urlencode(repo),
    );
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = r#ref {
        req_builder = req_builder.query(&[("ref", &param_value.to_string())]);
    }
    if let Some(ref param_value) = path {
        req_builder = req_builder.query(&[("path", &param_value.to_string())]);
    }
    if let Some(ref param_value) = depth {
        req_builder = req_builder.query(&[("depth", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref token) = configuration.bearer_access_token {
        req_builder = req_builder.bearer_auth(token.to_owned());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();

    if !status.is_client_error() && !status.is_server_error() {
        let text = resp.text().await?;
        serde_path_to_error::deserialize(&mut serde_json::Deserializer::from_str(&text))
            .map_err(Error::from)
    } else {
        let text = resp.text().await?;
        let entity: Option<GetContentError> = serde_json::from_str(&text).ok();
        Err(Error::ResponseError(ResponseContent {
            status,
            content: text,
            entity,
        }))
    }
}
