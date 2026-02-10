use crate::low_level::apis::{admin_api, Error};
use crate::models;

use super::OrgClient;

/// Client for API key management (`/{org}/api-keys`).
#[derive(Clone, Debug)]
pub struct ApiKeysClient<'a> {
    pub(super) org: &'a OrgClient<'a>,
}

impl ApiKeysClient<'_> {
    /// List all API keys (key values are not returned).
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn list(
        &self,
    ) -> Result<models::GetByOrgApiKeys200Response, Error<admin_api::GetByOrgApiKeysError>> {
        admin_api::get_by_org_api_keys(self.org.config, self.org.org).await
    }

    /// Create a new API key.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn create(
        &self,
        request: models::PostByOrgApiKeysRequest,
    ) -> Result<models::PostByOrgApiKeys201Response, Error<admin_api::PostByOrgApiKeysError>> {
        admin_api::post_by_org_api_keys(self.org.config, self.org.org, Some(request)).await
    }

    /// Revoke an API key by its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn revoke(
        &self,
        id: &str,
    ) -> Result<
        models::DeleteByOrgApiKeysById200Response,
        Error<admin_api::DeleteByOrgApiKeysByIdError>,
    > {
        admin_api::delete_by_org_api_keys_by_id(self.org.config, id, self.org.org).await
    }
}
