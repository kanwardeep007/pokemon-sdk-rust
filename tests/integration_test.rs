use pokemon_api_sdk::pokemon_sdk::PokemonSdkBuilder;

#[tokio::test]
async fn get_pokemon_detail_fetches_details_of_pokemon() {
    // Arrange
    let id_to_fetch = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new().build();

    // Act
    let pokemon_details = new_pokemon_sdk
        .pokemon()
        .pokemon_details(id_to_fetch.to_string())
        .await;

    // Assert
    assert_eq!(pokemon_details.id, id_to_fetch);
}

#[tokio::test]
async fn get_generation_detail_fetches_details_of_a_specific_pokemon_generation() {
    // Arrange
    let id_to_fetch = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new().build();

    // Act
    let generation_details = new_pokemon_sdk
        .games()
        .generation_details(id_to_fetch.to_string())
        .await;

    // Assert
    assert_eq!(generation_details.id, id_to_fetch);
    dbg!(&generation_details);
}
