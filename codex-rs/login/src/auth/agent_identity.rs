//! Agent identity stub module for Android build.

use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum AgentIdentityAuthError {
    #[error("Agent identity is disabled on Android target")]
    Disabled,
    #[error("Bootstrap unavailable")]
    BootstrapUnavailable {
        operation: String,
        attempts: u32,
        message: String,
    },
}

impl AgentIdentityAuthError {
    pub fn bootstrap_unavailable(_: &impl std::fmt::Debug) -> Option<&'static AgentIdentityAuthError> {
        None
    }
}

use std::sync::LazyLock;
use super::storage::AgentIdentityAuthRecord;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentIdentityAuth;

impl AgentIdentityAuth {
    pub async fn from_jwt(_: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug) -> Result<Self, std::io::Error> { Ok(Self) }
    pub async fn from_record(_: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug) -> Result<Self, std::io::Error> { Ok(Self) }
    pub fn account_id(&self) -> &str { "" }
    pub fn is_fedramp_account(&self) -> bool { false }
    pub fn email(&self) -> Option<&str> { None }
    pub fn chatgpt_user_id(&self) -> &str { "" }
    pub fn plan_type(&self) -> codex_protocol::account::PlanType { codex_protocol::account::PlanType::Free }
    pub fn run_task_id(&self) -> &str { "" }
    pub fn record(&self) -> &'static AgentIdentityAuthRecord {
        static RECORD: LazyLock<AgentIdentityAuthRecord> = LazyLock::new(AgentIdentityAuthRecord::default);
        &RECORD
    }
}

pub async fn verified_record_from_jwt(_: &str, _: &str, _: impl std::fmt::Debug) -> Result<AgentIdentityAuth, std::io::Error> {
    Ok(AgentIdentityAuth)
}

#[derive(Debug, Clone)]
pub struct ManagedChatGptAgentIdentityBinding {
    pub account_id: String,
    pub chatgpt_user_id: String,
    pub email: Option<String>,
    pub plan_type: codex_protocol::account::PlanType,
    pub chatgpt_account_is_fedramp: bool,
    pub access_token: String,
}

pub fn agent_identity_authapi_base_url(_: impl std::fmt::Debug) -> Result<String, std::io::Error> { Ok(String::new()) }
pub fn classify_bootstrap_error(_: impl std::fmt::Debug, _: impl std::fmt::Debug) -> std::io::Error {
    std::io::Error::other("disabled")
}
pub fn record_matches_managed_chatgpt_binding(_: impl std::fmt::Debug, _: impl std::fmt::Debug) -> bool { true }
pub fn record_needs_task_registration(_: impl std::fmt::Debug) -> bool { false }
pub async fn register_managed_chatgpt_agent_identity(_: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug) -> Result<AgentIdentityAuth, std::io::Error> { Ok(AgentIdentityAuth) }
pub fn require_agent_identity_authapi_base_url(_: impl std::fmt::Debug) -> Result<String, std::io::Error> { Ok(String::new()) }
