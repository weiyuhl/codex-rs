use crate::auth::SharedAuthProvider;
use crate::common::ResponseStream;
use crate::common::ResponsesWsRequest;
use crate::error::ApiError;
use crate::provider::Provider;
use codex_http_client::HttpClientFactory;

pub async fn responses_websocket(
    _provider: &Provider,
    _auth_provider: &SharedAuthProvider,
    _http_client_factory: &HttpClientFactory,
    _request: ResponsesWsRequest,
) -> Result<ResponseStream, ApiError> {
    Err(ApiError::Transport(codex_client::TransportError::Tonic(
        "websocket stream is disabled in embedded agent".to_string(),
    )))
}
