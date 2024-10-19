use crate::games_api::api::GamesApi;
use crate::pokemon_api::api::PokemonApi;
use std::sync::Arc;
use std::time::Duration;

use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryPolicy;

struct PokemonSdk {
    pokemon: PokemonApi,
    games: GamesApi,
}

struct PokemonSdkBuilder {
    http_client: reqwest::Client,
    retry_policy: Option<ExponentialBackoff>,
    timeout: Option<Duration>,
}

impl PokemonSdkBuilder {
    pub fn new() -> PokemonSdkBuilder {
        let client = reqwest::Client::new();
        let retry_policy = Some(ExponentialBackoff::builder().build_with_max_retries(3));
        let timeout = Duration::from_secs(3);
        PokemonSdkBuilder {
            http_client: client,
            retry_policy,
            timeout: Some(timeout),
        }
    }

    pub fn with_http_client(mut self, http_client: reqwest::Client) -> PokemonSdkBuilder {
        self.http_client = http_client;
        self
    }

    pub fn with_retry_policy(
        mut self,
        retry_policy: impl Into<Option<dyn RetryPolicy + Send + Sync + 'static>>,
    ) -> Self {
        self.retry_policy = retry_policy.into();
        self
    }
    pub fn with_retry_policy(mut self, retry_policy: impl RetryPolicy) -> PokemonSdkBuilder {
        self.retry_policy = retry_policy;
        self
    }

    fn build(&self) -> PokemonSdk {}
}
