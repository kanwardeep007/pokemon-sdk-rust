use reqwest::Url;
use reqwest_middleware::ClientWithMiddleware;
pub struct CoreHttpClient {
    pub client: ClientWithMiddleware,
    pub url: Url,
}
