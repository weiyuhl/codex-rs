mod auth;
mod bearer_auth_provider;
mod models_endpoint;
mod provider;

pub use auth::AgentIdentitySessionFallback;
pub use auth::ProviderAuthScope;
pub use auth::ResolvedProviderAuth;
pub use auth::auth_provider_from_auth;
pub use auth::auth_provider_from_auth_manager;
pub use auth::unauthenticated_auth_provider;
pub use bearer_auth_provider::BearerAuthProvider;
pub use bearer_auth_provider::BearerAuthProvider as CoreAuthProvider;
pub const AMAZON_BEDROCK_PROVIDER_ID: &str = "bedrock";
pub const CHATGPT_CODEX_BASE_URL: &str = "https://chatgpt.com";
pub mod codex_model_provider_info {
    #[derive(Clone, Debug, Default)]
    pub struct ModelProviderInfo {
        pub requires_openai_auth: bool,
        pub env_key: Option<String>,
        pub experimental_bearer_token: Option<String>,
        pub auth: Option<()>,
        pub aws: Option<()>,
        pub base_url: String,
    }
    impl ModelProviderInfo {
        pub fn to_api_provider(&self, _: impl std::fmt::Debug) -> Result<codex_api::Provider, codex_protocol::error::CodexErr> {
            Ok(codex_api::Provider {
                name: "openai".to_string(),
                base_url: self.base_url.clone(),
                query_params: None,
                headers: http::HeaderMap::new(),
                retry: codex_api::RetryConfig {
                    max_attempts: 3,
                    base_delay: std::time::Duration::from_millis(250),
                    retry_429: true,
                    retry_5xx: true,
                    retry_transport: true,
                },
                stream_idle_timeout: std::time::Duration::from_secs(30),
            })
        }
        pub fn is_amazon_bedrock(&self) -> bool { false }
        pub fn api_key(&self) -> Result<Option<String>, std::io::Error> { Ok(None) }
        pub fn has_command_auth(&self) -> bool { false }
    }
}
pub use codex_protocol::account::ProviderAccount;
pub use provider::ModelProvider;
pub use provider::ModelProviderFuture;
pub use provider::ProviderAccountError;
pub use provider::ProviderAccountResult;
pub use provider::ProviderAccountState;
pub use provider::ProviderCapabilities;
pub use provider::SharedModelProvider;
pub use provider::create_model_provider;
