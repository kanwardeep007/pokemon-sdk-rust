use crate::api_error;
use crate::core_client::CoreHttpClient;
use crate::games_api::model::Generation;
use api_error::Error;
use std::sync::Arc;

pub struct GamesApi {
    inner: Arc<CoreHttpClient>,
}

impl GamesApi {
    pub(crate) fn new(inner: Arc<CoreHttpClient>) -> Self {
        Self { inner }
    }
    pub async fn generation_details(&self, identifier: String) -> Result<Generation, Error> {
        let full_url = self.inner.url.join(&format!("generation/{}", identifier))?;

        let response: Generation = self.inner.client.get(full_url).send().await?.json().await?;

        return Ok(response);
    }
}
