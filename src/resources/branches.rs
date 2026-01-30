//! Branch operations.
//!
//! Access via [`MesaClient::branches`](crate::MesaClient::branches):
//!
//! ```rust,no_run
//! # use mesa_dev::{Mesa, MesaError, models::CreateBranchRequest};
//! # async fn run() -> Result<(), MesaError> {
//! # let client = Mesa::new("key");
//! let branch = client.branches("my-org", "my-repo").create(&CreateBranchRequest {
//!     name: "feature/new".to_owned(),
//!     from: "main".to_owned(),
//! }).await?;
//! # Ok(())
//! # }
//! ```

use std::sync::Arc;

use http::Method;

use crate::client::MesaClient;
use crate::error::MesaError;
use crate::http_client::HttpClient;
use crate::models::{
    Branch, CreateBranchRequest, ListBranchesResponse, PaginationParams, SuccessResponse,
};
use crate::pagination::PageStream;

/// Operations on branches within a repository.
pub struct BranchesResource<'c, C: HttpClient> {
    client: &'c MesaClient<C>,
    org: String,
    repo: String,
}

impl<'c, C: HttpClient> BranchesResource<'c, C> {
    pub(crate) fn new(client: &'c MesaClient<C>, org: String, repo: String) -> Self {
        Self { client, org, repo }
    }

    /// Create a new branch.
    ///
    /// # Errors
    ///
    /// Returns [`MesaError`] if the API request fails.
    pub async fn create(&self, req: &CreateBranchRequest) -> Result<Branch, MesaError> {
        let path = format!("/{}/{}/branches", self.org, self.repo);
        self.client
            .request(Method::POST, &path, &[], Some(req))
            .await
    }

    /// List branches with pagination parameters.
    ///
    /// # Errors
    ///
    /// Returns [`MesaError`] if the API request fails.
    pub async fn list(&self, params: &PaginationParams) -> Result<ListBranchesResponse, MesaError> {
        let path = format!("/{}/{}/branches", self.org, self.repo);
        let mut query = Vec::new();
        if let Some(ref cursor) = params.cursor {
            query.push(("cursor", cursor.as_str()));
        }
        if let Some(limit) = params.limit {
            let limit_str = limit.to_string();
            return self
                .client
                .request::<ListBranchesResponse>(
                    Method::GET,
                    &path,
                    &[&query[..], &[("limit", &limit_str)]].concat(),
                    None::<&()>,
                )
                .await;
        }
        self.client
            .request(Method::GET, &path, &query, None::<&()>)
            .await
    }

    /// Return a [`PageStream`] that iterates over all branches.
    #[must_use]
    pub fn list_all(&self) -> PageStream<C, ListBranchesResponse> {
        let path = format!("/{}/{}/branches", self.org, self.repo);
        PageStream::new(Arc::clone(&self.client.inner), path, Vec::new())
    }

    /// Delete a branch.
    ///
    /// # Errors
    ///
    /// Returns [`MesaError`] if the API request fails.
    pub async fn delete(&self, branch: &str) -> Result<SuccessResponse, MesaError> {
        let path = format!("/{}/{}/branches/{branch}", self.org, self.repo);
        self.client
            .request(Method::DELETE, &path, &[], None::<&()>)
            .await
    }
}
