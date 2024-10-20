use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Generation {
    pub id: i32,
    pub name: String,
    pub abilities: Vec<PokemonAbility>,
    pub main_region: Region,
    pub moves: Vec<Move>,
}

#[derive(Deserialize, Debug)]
pub struct Region {
    pub name: String,
    pub url: String,
}
#[derive(Deserialize, Debug)]
pub struct Move {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct PokemonAbility {
    pub is_hidden: bool,
    pub slot: i32,
    pub ability: String,
}

#[cfg(test)]
mod tests {
    use super::*;
}
