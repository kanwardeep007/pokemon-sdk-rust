use crate::api_error::Error;
use crate::core_client::CoreHttpClient;
use crate::pokemon_api::model::Pokemon;
use std::sync::Arc;

pub struct PokemonApi {
    inner: Arc<CoreHttpClient>,
}

impl PokemonApi {
    pub(crate) fn new(inner: Arc<CoreHttpClient>) -> Self {
        Self { inner }
    }

    pub async fn pokemon_details(&self, identifier: String) -> Result<Pokemon, Error> {
        let full_url = self.inner.url.join(&format!("pokemon/{}", identifier))?;

        let server_response = self.inner.client.get(full_url).send().await?;
        dbg!(&server_response.status());
        let response: Pokemon = server_response.json().await?;

        return Ok(response);
    }
}
