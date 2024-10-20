use crate::core_client::CoreHttpClient;
use crate::games_api::api::GamesApi;
use crate::pokemon_api::api::PokemonApi;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Url;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryPolicy;

pub struct PokemonSdk {
    pokemon: PokemonApi,
    games: GamesApi,
}

impl PokemonSdk {
    pub fn pokemon(&self) -> &PokemonApi {
        &self.pokemon
    }
    pub fn games(&self) -> &GamesApi {
        &self.games
    }
}

pub struct PokemonSdkBuilder {
    http_client: reqwest::Client,
    retry_policy: Option<Box<dyn RetryPolicy>>,
    timeout: Option<Duration>,
    server_url: Url,
}

impl PokemonSdkBuilder {
    pub fn new() -> PokemonSdkBuilder {
        let client = reqwest::Client::new();
        let retry_policy = Some(
            Box::new(ExponentialBackoff::builder().build_with_max_retries(3))
                as Box<dyn RetryPolicy>,
        );
        let timeout = Duration::from_secs(3);

        let url = Url::from_str("https://pokeapi.co/api/v2/").expect("Base url not a valid url");
        PokemonSdkBuilder {
            http_client: client,
            retry_policy,
            timeout: Some(timeout),
            server_url: url,
        }
    }

    pub fn with_http_client(mut self, http_client: reqwest::Client) -> PokemonSdkBuilder {
        self.http_client = http_client;
        self
    }

    pub fn with_retry_policy(
        mut self,
        retry_policy: impl Into<Option<Box<dyn RetryPolicy>>>,
    ) -> Self {
        self.retry_policy = retry_policy.into();
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout.into();
        self
    }

    pub fn build(self) -> PokemonSdk {
        let client_with_middleware =
            reqwest_middleware::ClientBuilder::new(self.http_client).build();
        let inner = Arc::new(CoreHttpClient {
            client: client_with_middleware,
            url: self.server_url,
        });

        PokemonSdk {
            pokemon: PokemonApi::new(inner.clone()),
            games: GamesApi::new(inner),
        }
    }
}
