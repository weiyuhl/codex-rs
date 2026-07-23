use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use ts_rs::TS;

pub const OPENAI_PROVIDER_ID: &str = "openai";
pub const OLLAMA_OSS_PROVIDER_ID: &str = "ollama";
pub const LMSTUDIO_OSS_PROVIDER_ID: &str = "lmstudio";
pub const AMAZON_BEDROCK_PROVIDER_ID: &str = "bedrock";
pub const LEGACY_OLLAMA_CHAT_PROVIDER_ID: &str = "ollama-chat";
pub const OLLAMA_CHAT_PROVIDER_REMOVED_ERROR: &str =
    "The 'ollama-chat' provider has been removed. Use 'ollama' instead.";

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
pub struct ModelProviderAuthInfo {
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
pub struct ModelProviderAwsAuthInfo {
    pub profile: Option<String>,
    pub region: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, JsonSchema, TS)]
pub struct ModelProviderInfo {
    #[serde(default)]
    pub name: String,
    pub base_url: Option<String>,
    pub env_key: Option<String>,
    pub env_key_instructions: Option<String>,
    pub experimental_bearer_token: Option<String>,
    pub http_headers: Option<HashMap<String, String>>,
    pub env_http_headers: Option<HashMap<String, String>>,
    pub auth: Option<ModelProviderAuthInfo>,
    pub aws: Option<ModelProviderAwsAuthInfo>,
}

impl Default for ModelProviderInfo {
    fn default() -> Self {
        Self::create_openai_provider(None)
    }
}

impl ModelProviderInfo {
    pub fn create_openai_provider(base_url: Option<String>) -> Self {
        Self {
            name: OPENAI_PROVIDER_ID.to_string(),
            base_url,
            env_key: Some("OPENAI_API_KEY".to_string()),
            env_key_instructions: None,
            experimental_bearer_token: None,
            http_headers: None,
            env_http_headers: None,
            auth: None,
            aws: None,
        }
    }

    pub fn create_amazon_bedrock_provider(aws: Option<ModelProviderAwsAuthInfo>) -> Self {
        Self {
            name: AMAZON_BEDROCK_PROVIDER_ID.to_string(),
            base_url: None,
            env_key: None,
            env_key_instructions: None,
            experimental_bearer_token: None,
            http_headers: None,
            env_http_headers: None,
            auth: None,
            aws,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
