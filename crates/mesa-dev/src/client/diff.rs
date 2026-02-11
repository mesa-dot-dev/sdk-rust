use crate::low_level::apis::{diffs_api, Error};
use crate::models;

use super::RepoClient;

/// Client for diff operations (`/{org}/{repo}/diff`).
#[derive(Clone, Debug)]
pub struct DiffClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl DiffClient<'_> {
    /// Retrieve the diff between two refs.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn get(
        &self,
        base: &str,
        head: &str,
    ) -> Result<models::GetByOrgByRepoDiff200Response, Error<diffs_api::GetByOrgByRepoDiffError>>
    {
        diffs_api::get_by_org_by_repo_diff(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            base,
            head,
        )
        .await
    }
}
