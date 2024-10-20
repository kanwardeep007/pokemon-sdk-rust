use crate::core_client::CoreHttpClient;
use crate::games_api::api::GamesApi;
use crate::pokemon_api::api::PokemonApi;
use crate::retry_policy_mod::RetryStrategy;
use anyhow::anyhow;
use reqwest::Url;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

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
    retry_strategy: Option<RetryStrategy>,
    server_url: Url,
}

impl PokemonSdkBuilder {
    pub fn new() -> Result<PokemonSdkBuilder, anyhow::Error> {
        let client = reqwest::Client::new();

        let url = Url::from_str("https://pokeapi.co/api/v2/").map_err(|e| {
            anyhow!(
                "Unable to generate URL from Server Url. Error: {} ",
                e.to_string()
            )
        })?;

        let backoff_retry_max_seconds = Duration::from_secs(3);
        Ok(PokemonSdkBuilder {
            http_client: client,
            retry_strategy: Some(RetryStrategy::ExponentialBackoffTimed {
                max_duration: backoff_retry_max_seconds,
            }),
            server_url: url,
        })
    }

    pub fn with_http_client(mut self, http_client: reqwest::Client) -> PokemonSdkBuilder {
        self.http_client = http_client;
        self
    }

    pub fn with_retry_policy(mut self, retry_policy: RetryStrategy) -> Self {
        self.retry_strategy = Some(retry_policy);
        self
    }

    pub fn with_url(mut self, url: Url) -> Self {
        self.server_url = url;
        self
    }

    pub fn get_url(&self) -> &Url {
        &self.server_url
    }

    pub fn get_retry_strategy(&self) -> Option<&RetryStrategy> {
        self.retry_strategy.as_ref()
    }
    pub fn get_client(&self) -> &reqwest::Client {
        &self.http_client
    }

    pub fn build(self) -> PokemonSdk {
        let mut client_with_middleware = reqwest_middleware::ClientBuilder::new(self.http_client);

        if let Some(inner_retry_policy) = self.retry_strategy {
            match inner_retry_policy {
                RetryStrategy::ExponentialBackoffTimed { max_duration } => {
                    let retry_policy =
                        ExponentialBackoff::builder().build_with_total_retry_duration(max_duration);
                    let retry_middleware = RetryTransientMiddleware::new_with_policy(retry_policy);
                    client_with_middleware = client_with_middleware.with(retry_middleware);
                }
            }
        }

        let built_client_with_middleware = client_with_middleware.build();
        let inner = Arc::new(CoreHttpClient {
            client: built_client_with_middleware,
            url: self.server_url,
        });

        PokemonSdk {
            pokemon: PokemonApi::new(inner.clone()),
            games: GamesApi::new(inner),
        }
    }
}

pub fn get_default_client(timeout: Option<Duration>) -> Result<reqwest::Client, anyhow::Error> {
    let mut client_builder = reqwest::ClientBuilder::new();
    if let Some(duration) = timeout {
        client_builder = client_builder.timeout(duration)
    }
    Ok(client_builder
        .build()
        .map_err(|e| anyhow!("Unable to generate client. Error: {}", e.to_string()))?)
}
