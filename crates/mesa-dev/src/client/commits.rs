use crate::low_level::apis::{commits_api, Error};
use crate::low_level::commits;
use crate::models;

use super::pagination::{paginate, PaginatedResponse};
use super::RepoClient;

use futures_core::Stream;

impl PaginatedResponse for models::GetByOrgByRepoCommits200Response {
    type Item = models::GetByOrgByRepoCommits200ResponseCommitsInner;

    fn items(self) -> Vec<Self::Item> {
        self.commits
    }

    fn next_cursor(&self) -> Option<&str> {
        self.next_cursor.as_deref()
    }

    fn has_more(&self) -> bool {
        self.has_more
    }
}

/// Client for commit operations (`/{org}/{repo}/commits`).
#[derive(Clone, Debug)]
pub struct CommitsClient<'a> {
    pub(super) repo: &'a RepoClient<'a>,
}

impl<'a> CommitsClient<'a> {
    /// List commits, optionally filtered by ref.
    ///
    /// Returns an async stream that automatically paginates through all results,
    /// yielding one commit at a time. Pass `limit` to control the page size of
    /// each underlying API request, or `None` for the server default.
    pub fn list(
        &self,
        r#ref: Option<&'a str>,
        limit: Option<u8>,
    ) -> impl Stream<
        Item = Result<
            models::GetByOrgByRepoCommits200ResponseCommitsInner,
            Error<commits_api::GetByOrgByRepoCommitsError>,
        >,
    > + 'a {
        tracing::debug!(
            org = self.repo.org.org,
            repo = self.repo.repo,
            r#ref,
            limit,
            "listing commits"
        );
        let config = self.repo.org.config;
        let org = self.repo.org.org;
        let repo = self.repo.repo;

        paginate(limit, move |cursor, lim| async move {
            commits_api::get_by_org_by_repo_commits(
                config,
                org,
                repo,
                cursor.as_deref(),
                lim,
                r#ref,
            )
            .await
        })
    }

    /// Get a specific commit by its SHA.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[tracing::instrument(skip(self), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn get(
        &self,
        sha: &str,
    ) -> Result<
        models::GetByOrgByRepoCommitsBySha200Response,
        Error<commits_api::GetByOrgByRepoCommitsByShaError>,
    > {
        commits_api::get_by_org_by_repo_commits_by_sha(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            Some(sha),
        )
        .await
    }

    /// Create a commit with file operations.
    ///
    /// Uses the hand-written implementation to work around the missing
    /// `Upsert` action variant in the generated code.
    ///
    /// # Errors
    ///
    /// Returns an error if the API request fails.
    #[allow(clippy::too_many_arguments)]
    #[tracing::instrument(skip(self, author, files), fields(org = self.repo.org.org, repo = self.repo.repo), err(Debug))]
    pub async fn create(
        &self,
        branch: &str,
        message: &str,
        author: &commits::CommitAuthor,
        files: &[commits::CommitFile],
        base_sha: Option<&str>,
    ) -> Result<commits::CommitResponse, Error<commits::CreateCommitError>> {
        commits::create_commit(
            self.repo.org.config,
            self.repo.org.org,
            self.repo.repo,
            branch,
            message,
            author,
            files,
            base_sha,
        )
        .await
    }
}
