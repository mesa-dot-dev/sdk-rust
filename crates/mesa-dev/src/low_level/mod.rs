//! Lower-level API access and codegen workarounds.
//!
//! Most users should prefer the ergonomic [`MesaClient`](crate::MesaClient)
//! wrapper. This module exposes the generated API functions directly and
//! provides hand-written replacements for endpoints where the `OpenAPI` code
//! generator produces incorrect types.

pub mod commits;
pub mod content;

/// OpenAPI-generated API client modules.
///
/// Each submodule corresponds to a group of API endpoints (e.g.,
/// [`repos_api`], [`branches_api`]).
///
/// The [`configuration`] module contains the
/// [`Configuration`](configuration::Configuration) type used to
/// configure authentication and the base URL.
pub mod apis {
    pub use mesa_dev_oapi::apis::{
        admin_api, agent_blame_api, branches_api, commits_api, configuration, diffs_api, lfs_api,
        org_api, repos_api, webhooks_api, Error, ResponseContent,
    };
}
