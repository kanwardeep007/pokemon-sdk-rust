use pokemon_api_sdk::pokemon_sdk::PokemonSdkBuilder;

#[tokio::test]
async fn get_pokemon_detail_fetches_details_of_pokemon() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new().build();

    // Act
    let pokemon_details = new_pokemon_sdk
        .pokemon()
        .pokemon_details(id.to_string())
        .await;

    // Assert
    assert_eq!(pokemon_details.id, 1);
}

#[tokio::test]
async fn get_generation_detail_fetches_details_of_a_specific_pokemon_generation() {
    // Arrange
    let id = 1;
    let new_pokemon_sdk = PokemonSdkBuilder::new().build();

    // Act
    let generation_details = new_pokemon_sdk
        .games()
        .generation_details(id.to_string())
        .await;

    // Assert
    assert_eq!(generation_details.id, 1);
    dbg!(&generation_details);
}
