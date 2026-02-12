use crate::low_level::apis::{lfs_api, Error};
use crate::models;

use super::RepoClient;

/// Client for LFS operations (`/{org}/{repo}/lfs`).
#[derive(Clone, Debug)]
pub struct LfsClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl LfsClient<'_> {
    /// Request pre-signed URLs to upload large files to LFS storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self, request), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn upload(
        &self,
        request: models::PostByOrgByRepoLfsObjectsRequest,
    ) -> Result<
        models::PostByOrgByRepoLfsObjects200Response,
        Error<lfs_api::PostByOrgByRepoLfsObjectsError>,
    > {
        lfs_api::post_by_org_by_repo_lfs_objects(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            Some(request),
        )
        .await
    }

    /// Request pre-signed URLs to download large files from LFS storage.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self, request), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn download(
        &self,
        request: models::PostByOrgByRepoLfsObjectsDownloadRequest,
    ) -> Result<
        models::PostByOrgByRepoLfsObjects200Response,
        Error<lfs_api::PostByOrgByRepoLfsObjectsDownloadError>,
    > {
        lfs_api::post_by_org_by_repo_lfs_objects_download(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            Some(request),
        )
        .await
    }
}
