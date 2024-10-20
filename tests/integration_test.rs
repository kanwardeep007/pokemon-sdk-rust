use std::time::Duration;

use pokemon_api_sdk::pokemon_sdk::PokemonSdkBuilder;

#[tokio::test]
async fn get_pokemon_detail_fetches_details_of_pokemon() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new()
        .expect("Unable to build Pokemon SDK")
        .with_http_client(
            pokemon_api_sdk::pokemon_sdk::get_default_client(Some(Duration::from_secs(3))).unwrap(),
        )
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
        .with_http_client(
            pokemon_api_sdk::pokemon_sdk::get_default_client(Some(Duration::from_secs(3))).unwrap(),
        )
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
