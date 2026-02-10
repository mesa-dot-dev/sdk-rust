use crate::low_level::apis::{repos_api, Error};
use crate::models;

use super::RepoClient;

/// Client for sync operations (`/{org}/{repo}/sync`).
#[derive(Clone, Debug)]
pub struct SyncClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl SyncClient<'_> {
    /// Get sync status with upstream.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn status(
        &self,
    ) -> Result<models::GetByOrgByRepoSync200Response, Error<repos_api::GetByOrgByRepoSyncError>>
    {
        repos_api::get_by_org_by_repo_sync(self.repo.org.config, self.repo.org.org, self.repo.repo)
            .await
    }

    /// Trigger a sync from the upstream repository (waits for completion).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn trigger(
        &self,
    ) -> Result<models::DeleteByOrgApiKeysById200Response, Error<repos_api::PostByOrgByRepoSyncError>>
    {
        repos_api::post_by_org_by_repo_sync(self.repo.org.config, self.repo.org.org, self.repo.repo)
            .await
    }
}
