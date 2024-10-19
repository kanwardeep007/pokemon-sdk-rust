use reqwest_middleware::ClientWithMiddleware;
pub struct CoreHttpClient {
    pub client: ClientWithMiddleware,
}
