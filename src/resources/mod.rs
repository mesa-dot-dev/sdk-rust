//! Resource namespaces for the Mesa API.
//!
//! Each resource type is accessed through a method on [`MesaClient`](crate::MesaClient):
//!
//! | Resource | Accessor | Scope |
//! |----------|----------|-------|
//! | [`ReposResource`] | `client.repos(org)` | Organization |
//! | [`BranchesResource`] | `client.branches(org, repo)` | Repository |
//! | [`CommitsResource`] | `client.commits(org, repo)` | Repository |
//! | [`ContentResource`] | `client.content(org, repo)` | Repository |
//! | [`DiffsResource`] | `client.diffs(org, repo)` | Repository |
//! | [`AdminResource`] | `client.admin(org)` | Organization |

mod admin;
mod branches;
mod commits;
mod content;
mod diffs;
mod repos;

pub use admin::AdminResource;
pub use branches::BranchesResource;
pub use commits::{CommitsResource, ListCommitsParams};
pub use content::ContentResource;
pub use diffs::DiffsResource;
pub use repos::ReposResource;
