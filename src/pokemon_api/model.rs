use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pokemon {
    id: i32,
    name: String,
    base_experience: i32,
    height: i32,
    weight: i32,
}

#[derive(Deserialize, Debug)]
struct PokemonAbility {
    is_hidden: bool,
    slot: i32,
    ability: String,
}
