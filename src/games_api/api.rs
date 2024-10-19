use crate::client::CoreHttpClient;
use crate::games_api::model::Generation;
use std::sync::Arc;

pub struct GamesApi {
    inner: Arc<CoreHttpClient>,
}

impl GamesApi {
    pub fn generation_details(&self) -> Generation {
        let response = self.inner.client.get(url);
        todo!();
    }
}
