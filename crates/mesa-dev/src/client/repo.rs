use crate::low_level::apis::{repos_api, Error};
use crate::models;

use super::{
    AnalyticsClient, BranchesClient, CommitsClient, ContentClient, DiffClient, LfsClient,
    OrgClient, SyncClient, WebhooksClient,
};

/// Client scoped to a specific repository (`/{org}/{repo}`).
#[derive(Clone, Debug)]
pub struct RepoClient<'a> {
    pub(super) org: &'a OrgClient<'a>,
    pub(super) repo: &'a str,
}

impl RepoClient<'_> {
    /// Get repository metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.org.org, repo = self.repo), err(Debug))]
    pub async fn get(
        &self,
    ) -> Result<models::PostByOrgRepos201Response, Error<repos_api::GetByOrgByRepoError>> {
        repos_api::get_by_org_by_repo(self.org.config, self.org.org, self.repo).await
    }

    /// Permanently delete this repository and all its data.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.org.org, repo = self.repo), err(Debug))]
    pub async fn delete(
        &self,
    ) -> Result<models::DeleteByOrgApiKeysById200Response, Error<repos_api::DeleteByOrgByRepoError>>
    {
        repos_api::delete_by_org_by_repo(self.org.config, self.org.org, self.repo).await
    }

    /// Update repository name or upstream configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self, request), fields(org = self.org.org, repo = self.repo), err(Debug))]
    pub async fn update(
        &self,
        request: models::PatchByOrgByRepoRequest,
    ) -> Result<models::PostByOrgRepos201Response, Error<repos_api::PatchByOrgByRepoError>> {
        repos_api::patch_by_org_by_repo(self.org.config, self.org.org, self.repo, Some(request))
            .await
    }

    /// Access branch operations.
    #[must_use]
    pub fn branches(&self) -> BranchesClient<'_> {
        BranchesClient { repo: self }
    }

    /// Access commit operations.
    #[must_use]
    pub fn commits(&self) -> CommitsClient<'_> {
        CommitsClient { repo: self }
    }

    /// Access content operations.
    #[must_use]
    pub fn content(&self) -> ContentClient<'_> {
        ContentClient { repo: self }
    }

    /// Access diff operations.
    #[must_use]
    pub fn diff(&self) -> DiffClient<'_> {
        DiffClient { repo: self }
    }

    /// Access sync operations.
    #[must_use]
    pub fn sync(&self) -> SyncClient<'_> {
        SyncClient { repo: self }
    }

    /// Access webhook operations.
    #[must_use]
    pub fn webhooks(&self) -> WebhooksClient<'_> {
        WebhooksClient { repo: self }
    }

    /// Access LFS operations.
    #[must_use]
    pub fn lfs(&self) -> LfsClient<'_> {
        LfsClient { repo: self }
    }

    /// Access analytics and AI attribution.
    #[must_use]
    pub fn analytics(&self) -> AnalyticsClient<'_> {
        AnalyticsClient { repo: self }
    }
}
