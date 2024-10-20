use crate::core_client::CoreHttpClient;
use crate::games_api::model::Generation;
use std::sync::Arc;

pub struct GamesApi {
    inner: Arc<CoreHttpClient>,
}

impl GamesApi {
    pub(crate) fn new(inner: Arc<CoreHttpClient>) -> Self {
        Self { inner }
    }
    pub async fn generation_details(&self, identifier: String) -> Generation {
        let full_url = self
            .inner
            .url
            .join(&format!("generation/{}", identifier))
            .unwrap();

        let response: Generation = self
            .inner
            .client
            .get(full_url)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        return response;
    }
}

#[cfg(test)]
mod tests {
    use reqwest::Url;
    use serde_json::json;
    use wiremock::{
        matchers::{method, path},
        Mock, MockServer, ResponseTemplate,
    };

    use super::*;

    async fn mock_client_and_server() -> (GamesApi, MockServer) {
        let mock_server = MockServer::start().await;

        let client = reqwest_middleware::ClientBuilder::new(reqwest::Client::new()).build();
        let inner = CoreHttpClient {
            client,
            url: Url::parse("test.com").expect("Unable to generate URL"),
        };

        (GamesApi::new(Arc::new(inner)), mock_server)
    }

    #[tokio::test]
    async fn list() {
        let (api, mock_server) = mock_client_and_server().await;

        Mock::given(method("GET"))
            .and(path("/generation"))
            .respond_with(ResponseTemplate::new(503))
            .expect(1)
            .mount(&mock_server)
            .await;

        let merchant_accounts = api.list().await.unwrap();

        assert_eq!(
            merchant_accounts,
            vec![MerchantAccount {
                id: "merchant-account-id".to_string(),
                currency: Currency::Gbp,
                account_identifiers: vec![AccountIdentifier::SortCodeAccountNumber {
                    sort_code: "sort-code".to_string(),
                    account_number: "account-number".to_string()
                }],
                available_balance_in_minor: 100,
                current_balance_in_minor: 200,
                account_holder_name: "Mr. Holder".to_string()
            }]
        );
    }
}
