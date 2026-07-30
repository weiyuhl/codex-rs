use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StoredOAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
    pub client_id: String,
    pub token_response: (rmcp::transport::auth::OAuthTokenResponse,),
}

impl Default for StoredOAuthTokens {
    fn default() -> Self {
        Self {
            access_token: String::new(),
            refresh_token: None,
            expires_at: None,
            client_id: String::new(),
            token_response: (serde_json::from_str("{}").unwrap(),),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct WrappedOAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
}

#[derive(Clone, Copy, Debug)]
pub struct ResolvedOAuthCredentialStore;

impl ResolvedOAuthCredentialStore {
    pub async fn load(&self, _: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any) -> Result<Option<StoredOAuthTokens>> {
        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub struct ResolvedOAuthTokens {
    pub tokens: StoredOAuthTokens,
    pub store: ResolvedOAuthCredentialStore,
}

pub async fn resolve_oauth_tokens_from_store_policy(_: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any) -> Result<Option<ResolvedOAuthTokens>> {
    Ok(None)
}

#[derive(Clone, Debug)]
pub struct OAuthPersistor;

impl OAuthPersistor {
    pub fn new(_: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any, _: impl std::any::Any) -> Self { Self }
    pub async fn persist_if_needed(&self) -> Result<()> {
        Ok(())
    }
    pub async fn refresh_if_needed(&self) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StoredOAuthTokenStatus {
    Valid,
    Expired,
    Missing,
    Usable,
    AuthorizationRequired,
}

pub fn oauth_token_status(_: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug, _: impl std::fmt::Debug) -> Result<StoredOAuthTokenStatus, std::io::Error> {
    Ok(StoredOAuthTokenStatus::Missing)
}

pub async fn save_oauth_tokens(_server_name: &str, _tokens: &StoredOAuthTokens) -> Result<()> {
    Ok(())
}

pub async fn delete_oauth_tokens(_server_name: &str) -> Result<()> {
    Ok(())
}
