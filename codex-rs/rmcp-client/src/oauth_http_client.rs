#[derive(Clone, Debug)]
pub struct OAuthHttpClientAdapter;

impl OAuthHttpClientAdapter {
    pub fn new(_: impl std::any::Any, _: impl std::any::Any) -> Self { Self }
}

impl rmcp::transport::auth::OAuthHttpClient for OAuthHttpClientAdapter {
    fn execute<'a>(&'a self, _: rmcp::transport::auth::OAuthHttpRequest) -> futures::future::BoxFuture<'a, Result<oauth2::endpoint::HttpResponse, rmcp::transport::auth::OAuthHttpClientError>> {
        Box::pin(async { Err(rmcp::transport::auth::OAuthHttpClientError::Request(reqwest::Error::from(std::io::Error::other("disabled")))) })
    }
}
