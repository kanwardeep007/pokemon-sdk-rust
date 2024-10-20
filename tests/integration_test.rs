use std::str::FromStr;

use pokemon_api_sdk::pokemon_sdk::PokemonSdkBuilder;
use serde_json::json;
use url::Url;
use wiremock::matchers::path;
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn get_pokemon_detail_fetches_details_of_pokemon() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new()
        .expect("Unable to build Pokemon SDK")
        .build();

    // Act
    let pokemon_details = new_pokemon_sdk
        .pokemon()
        .pokemon_details(id.to_string())
        .await;

    // Assert
    dbg!(&pokemon_details);
    assert!(pokemon_details.is_ok());
    assert_eq!(pokemon_details.unwrap().id, 1);
}

#[tokio::test]
async fn get_generation_detail_fetches_details_of_a_specific_pokemon_generation() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new()
        .expect("Unable to build Pokemon SDK")
        .build();

    // Act
    let generation_details = new_pokemon_sdk
        .games()
        .generation_details(id.to_string())
        .await;

    // Assert
    assert!(generation_details.is_ok());
    assert_eq!(generation_details.unwrap().id, 1);
}

#[tokio::test]
async fn get_pokemon_retry_fetching_pokemon_when_4xx_error_occurs() {
    // Arrange
    let mock_server = MockServer::start().await;

    Mock::given(path("/pokemon/1"))
        .respond_with(ResponseTemplate::new(429)) // Too Many Requests
        .expect(1)
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    let mock_server_uri = mock_server.uri();
    let id = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new()
        .expect("Unable to build Pokemon SDK")
        .with_url(Url::from_str(&mock_server.uri()).unwrap())
        .build();
    let pokemon_details = new_pokemon_sdk
        .pokemon()
        .pokemon_details(id.to_string())
        .await;

    Mock::given(path("/pokemon/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": 1,
            "name": "pikachu",
            "base_experience": 11,
            "height": 10,
            "weight": 10,
            "abilities":[
                {
                    "is_hidden": false,
                    "slot": 2,
                    "ability": {
                        "name": "randomability",
                        "url": "www.random.com",
                    },
                }
            ]
        })))
        .expect(1)
        .up_to_n_times(2)
        .mount(&mock_server)
        .await;

    // Act
    let pokemon_details = new_pokemon_sdk
        .pokemon()
        .pokemon_details(id.to_string())
        .await;

    // Assert
    dbg!(mock_server.received_requests().await);
    dbg!(&pokemon_details);
    assert!(pokemon_details.is_ok());
    assert_eq!(pokemon_details.unwrap().id, 1);
}
