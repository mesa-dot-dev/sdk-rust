use crate::low_level::apis::{repos_api, Error};
use crate::models;

use super::pagination::{paginate, PaginatedResponse};
use super::{OrgClient, RepoClient};

use futures_core::Stream;

impl PaginatedResponse for models::GetByOrgRepos200Response {
    type Item = models::GetByOrgRepos200ResponseReposInner;

    fn items(self) -> Vec<Self::Item> {
        self.repos
    }

    fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }

    fn has_more(&self) -> bool {
        self.has_more
    }
}

/// Client for listing and creating repositories (`/{org}/repos`).
#[derive(Clone, Debug)]
pub struct ReposClient<'a> {
    pub(super) org: &'a OrgClient<'a>,
}

impl<'a> ReposClient<'a> {
    /// List repositories in the organization.
    ///
    /// Returns an async stream that automatically paginates through all results,
    /// yielding one repository at a time. Pass `limit` to control the page size
    /// of each underlying API request, or `None` for the server default.
    pub fn list(
        &self,
        limit: Option<u8>,
    ) -> impl Stream<
        Item = Result<
            models::GetByOrgRepos200ResponseReposInner,
            Error<repos_api::GetByOrgReposError>,
        >,
    > + 'a {
        tracing::debug!(org = self.org.org, limit, "listing repos");
        let config = self.org.config;
        let org = self.org.org;

        paginate(limit, move |cursor, lim| async move {
            repos_api::get_by_org_repos(config, org, cursor.as_deref(), lim).await
        })
    }

    /// Create a new repository.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self, request), fields(org = self.org.org), err(Debug))]
    pub async fn create(
        &self,
        request: models::PostByOrgReposRequest,
    ) -> Result<models::PostByOrgRepos201Response, Error<repos_api::PostByOrgReposError>> {
        repos_api::post_by_org_repos(self.org.config, self.org.org, Some(request)).await
    }

    /// Navigate to a specific repository.
    #[must_use]
    pub fn at(&self, name: &'a str) -> RepoClient<'a> {
        RepoClient {
            org: self.org,
            repo: name,
        }
    }
}
