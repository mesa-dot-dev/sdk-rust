use crate::low_level::apis::Error;
use crate::low_level::content;

use super::RepoClient;

/// Client for content operations (`/{org}/{repo}/content`).
#[derive(Clone, Debug)]
pub struct ContentClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl ContentClient<'_> {
    /// Get file content or directory listing at a path.
    ///
    /// Uses the hand-written implementation for correct `anyOf` handling.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn get(
        &self,
        r#ref: Option<&str>,
        path: Option<&str>,
        depth: Option<u64>,
    ) -> Result<content::Content, Error<content::GetContentError>> {
        content::get_content(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            r#ref,
            path,
            depth,
        )
        .await
    }
}
