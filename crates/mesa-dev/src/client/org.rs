use crate::low_level::apis::{configuration::Configuration, org_api, Error};
use crate::models;

use super::{ApiKeysClient, ReposClient};

/// Client scoped to an organization (`/{org}`).
#[derive(Clone, Debug)]
pub struct OrgClient<'a> {
    pub(super) config: &'a Configuration,
    pub(super) org: &'a str,
}

impl OrgClient<'_> {
    /// Get organization metadata and repository counts.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.org), err(Debug))]
    pub async fn get(&self) -> Result<models::GetByOrg200Response, Error<org_api::GetByOrgError>> {
        org_api::get_by_org(self.config, self.org).await
    }

    /// Access repository listing and creation.
    #[must_use]
    pub fn repos(&self) -> ReposClient<'_> {
        ReposClient { org: self }
    }

    /// Access API key management.
    #[must_use]
    pub fn api_keys(&self) -> ApiKeysClient<'_> {
        ApiKeysClient { org: self }
    }
}
