//! Agent identity stub module for Android build.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AgentIdentityAuthError {
    #[error("Agent identity is disabled on Android target")]
    Disabled,
}
