//! Common response types.

use serde::Deserialize;

/// A generic success response from the API.
///
/// Returned by operations that don't have a meaningful response body,
/// such as delete operations.
#[derive(Debug, Clone, Deserialize)]
pub struct SuccessResponse {
    /// Whether the operation succeeded.
    pub success: bool,
}
