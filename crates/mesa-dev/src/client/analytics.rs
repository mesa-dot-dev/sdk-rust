use crate::low_level::apis::{agent_blame_api, Error};
use crate::models;

use super::RepoClient;

/// Client for analytics and AI attribution (`/{org}/{repo}/analytics`).
#[derive(Clone, Debug)]
pub struct AnalyticsClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl AnalyticsClient<'_> {
    /// Get repository analytics.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn get(
        &self,
        period: Option<&str>,
    ) -> Result<
        models::GetByOrgByRepoAnalytics200Response,
        Error<agent_blame_api::GetByOrgByRepoAnalyticsError>,
    > {
        agent_blame_api::get_by_org_by_repo_analytics(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            period,
        )
        .await
    }

    /// Trigger a full re-aggregation of repository analytics.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn refresh(
        &self,
    ) -> Result<
        models::GetByOrgByRepoAnalytics200Response,
        Error<agent_blame_api::PostByOrgByRepoAnalyticsRefreshError>,
    > {
        agent_blame_api::post_by_org_by_repo_analytics_refresh(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
        )
        .await
    }

    /// Get AI attribution data between two refs.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    pub async fn agentblame(
        &self,
        base: &str,
        head: &str,
    ) -> Result<
        models::GetByOrgByRepoAgentblame200Response,
        Error<agent_blame_api::GetByOrgByRepoAgentblameError>,
    > {
        agent_blame_api::get_by_org_by_repo_agentblame(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            base,
            head,
        )
        .await
    }
}
