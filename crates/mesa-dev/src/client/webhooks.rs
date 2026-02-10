use crate::low_level::apis::{webhooks_api, Error};
use crate::models;

use super::RepoClient;

/// Client for webhook operations (`/{org}/{repo}/webhooks`).
#[derive(Clone, Debug)]
pub struct WebhooksClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl WebhooksClient<'_> {
    /// List webhooks for the repository.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn list(
        &self,
    ) -> Result<
        models::GetByOrgByRepoWebhooks200Response,
        Error<webhooks_api::GetByOrgByRepoWebhooksError>,
    > {
        webhooks_api::get_by_org_by_repo_webhooks(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
        )
        .await
    }

    /// Create a webhook.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn create(
        &self,
        request: models::PostByOrgByRepoWebhooksRequest,
    ) -> Result<
        models::PostByOrgByRepoWebhooks201Response,
        Error<webhooks_api::PostByOrgByRepoWebhooksError>,
    > {
        webhooks_api::post_by_org_by_repo_webhooks(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            Some(request),
        )
        .await
    }

    /// Delete a webhook by its ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn delete(
        &self,
        webhook_id: &str,
    ) -> Result<
        models::DeleteByOrgApiKeysById200Response,
        Error<webhooks_api::DeleteByOrgByRepoWebhooksByWebhookIdError>,
    > {
        webhooks_api::delete_by_org_by_repo_webhooks_by_webhook_id(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            webhook_id,
        )
        .await
    }
}
