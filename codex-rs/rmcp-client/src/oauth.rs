use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Default)]
pub struct StoredOAuthTokens {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct WrappedOAuthTokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: Option<i64>,
}

pub async fn save_oauth_tokens(_server_name: &str, _tokens: &StoredOAuthTokens) -> Result<()> {
    Ok(())
}

pub async fn delete_oauth_tokens(_server_name: &str) -> Result<()> {
    Ok(())
}
