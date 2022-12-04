use super::{
    faction::Faction,
    outfit::{Outfit, OutfitBy},
    title::Title,
    world::World,
};
use crate::{census::census_get, query};
use async_graphql::{ComplexObject, Object, OneofObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/// Character
/// Source: https://census.daybreakgames.com/get/ps2:v2/character
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct Character {
    #[graphql(name = "id")]
    pub character_id: String,
    pub head_id: String,
    pub certs: Certs,
    pub battle_rank: BattleRank,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub prestige_level: u8,

    // Special resolver
    #[graphql(skip)]
    pub name: Name,

    /// title_id resolves to Title
    #[graphql(skip)]
    title_id: String,

    /// faction_id resolves to Faction
    #[graphql(skip)]
    faction_id: String,

    #[graphql(skip)]
    outfit: Option<PartialCharacterOutfit>,
    #[graphql(skip)]
    world_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CharacterResponse {
    character_list: Vec<Character>,
}

#[derive(OneofObject, Debug)]
pub enum CharacterBy {
    Id(String),
    Name(String),
}

impl Character {
    pub async fn query(by: CharacterBy) -> Result<Character, String> {
        let (field, value) = match by {
            CharacterBy::Id(id) => ("character_id", id),
            CharacterBy::Name(name) => ("name.first_lower", name.to_lowercase()),
        };

        let response = census_get::<CharacterResponse>(
            "character",
            query!(
                (field, value),
                ("c:resolve", "outfit(outfit_id),world(world_id)".to_string()),
            ),
            None,
        )
        .await
        .unwrap()
        .character_list
        .pop();

        match response {
            Some(character) => Ok(character),
            None => Err("No character found".to_string()),
        }
    }
}

#[ComplexObject]
impl Character {
    /// Character name normalized from `name: {first, first_lower}` structure.
    /// If you prefer lower, ask as `name(lower: true)`.
    async fn name(&self, #[graphql(default = false)] lower: bool) -> String {
        if !lower {
            self.name.first.clone()
        } else {
            self.name.first_lower.clone()
        }
    }

    async fn title(&self) -> Option<Title> {
        Title::query(self.title_id.clone()).await.ok()
    }
    async fn faction(&self) -> Faction {
        Faction::query(self.faction_id.clone()).await.unwrap()
    }
    async fn world(&self) -> World {
        World::query(self.world_id.clone(), false).await.unwrap()
    }
    async fn outfit(&self) -> Option<Outfit> {
        match self.outfit.as_ref() {
            Some(outfit) => Some(
                Outfit::query(OutfitBy::Id(outfit.outfit_id.clone()))
                    .await
                    .unwrap(),
            ),
            None => None,
        }
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, Default)]
pub struct Certs {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub earned_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub gifted_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub spent_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub available_points: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub percent_to_next: f32,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct Name {
    pub first: String,
    pub first_lower: String,
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone, Default)]
pub struct BattleRank {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub percent_to_next: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PartialCharacterOutfit {
    outfit_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PartialCharacterWorld {
    world_id: String,
}

#[derive(Default)]
pub struct CharacterQuery;

#[Object]
impl CharacterQuery {
    /// Returns a graph for the character with the given ID or name (case-insensitive).
    /// Example: `character(by: { name: "wrel" })`
    /// Name can also start with `^` to match the beginning of the name, ex: `^wre`, but will only return the first result.
    async fn character(&self, by: CharacterBy) -> Result<Character, String> {
        Character::query(by).await
    }
}
