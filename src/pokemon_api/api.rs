use crate::client::CoreHttpClient;
use crate::pokemon_api::model::Pokemon;
use std::sync::Arc;

pub struct PokemonApi {
    inner: Arc<CoreHttpClient>,
}

impl PokemonApi {
    pub fn pokemon_details(&self) -> Pokemon {
        let response = self.inner.client.get(url);
        todo!();
    }
}
