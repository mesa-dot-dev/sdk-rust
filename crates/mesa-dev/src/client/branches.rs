use crate::low_level::apis::{branches_api, Error};
use crate::models;

use super::pagination::{paginate, PaginatedResponse};
use super::RepoClient;

use futures_core::Stream;

impl PaginatedResponse for models::GetByOrgByRepoBranches200Response {
    type Item = models::GetByOrgByRepoBranches200ResponseBranchesInner;

    fn items(self) -> Vec<Self::Item> {
        self.branches
    }

    fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }

    fn has_more(&self) -> bool {
        self.has_more
    }
}

/// Client for branch operations (`/{org}/{repo}/branches`).
#[derive(Clone, Debug)]
pub struct BranchesClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl<'a> BranchesClient<'a> {
    /// List all branches in the repository.
    ///
    /// Returns an async stream that automatically paginates through all results,
    /// yielding one branch at a time. Pass `limit` to control the page size of
    /// each underlying API request, or `None` for the server default.
    pub fn list(
        &self,
        limit: Option<u8>,
    ) -> impl Stream<
        Item = Result<
            models::GetByOrgByRepoBranches200ResponseBranchesInner,
            Error<branches_api::GetByOrgByRepoBranchesError>,
        >,
    > + 'a {
        tracing::debug!(
            org = self.repo.org.org,
            repo = self.repo.repo,
            limit,
            "listing branches"
        );
        let config = self.repo.org.config;
        let org = self.repo.org.org;
        let repo = self.repo.repo;

        paginate(limit, move |cursor, lim| async move {
            branches_api::get_by_org_by_repo_branches(config, org, repo, cursor.as_deref(), lim)
                .await
        })
    }

    /// Create a new branch from an existing ref.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self, request), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn create(
        &self,
        request: models::PostByOrgByRepoBranchesRequest,
    ) -> Result<
        models::PostByOrgByRepoBranches201Response,
        Error<branches_api::PostByOrgByRepoBranchesError>,
    > {
        branches_api::post_by_org_by_repo_branches(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            Some(request),
        )
        .await
    }

    /// Delete a branch by name.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn delete(
        &self,
        branch: &str,
    ) -> Result<
        models::DeleteByOrgApiKeysById200Response,
        Error<branches_api::DeleteByOrgByRepoBranchesByBranchError>,
    > {
        branches_api::delete_by_org_by_repo_branches_by_branch(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            Some(branch),
        )
        .await
    }
}
