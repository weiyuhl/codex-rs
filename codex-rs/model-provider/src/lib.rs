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
            Ok(codex_api::Provider::OpenAi)
        }
        pub fn is_amazon_bedrock(&self) -> bool { false }
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
