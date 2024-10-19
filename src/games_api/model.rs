use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Generation {
    id: i32,
    name: String,
    abilities: Vec<PokemonAbility>,
    main_region: Region,
    moves: Vec<Move>,
}

#[derive(Deserialize, Debug)]
struct Region {
    name: String,
    url: String,
}
#[derive(Deserialize, Debug)]
struct Move {
    name: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct PokemonAbility {
    is_hidden: bool,
    slot: i32,
    ability: String,
}
