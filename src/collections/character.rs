use super::{
    faction::Faction,
    outfit::{Outfit, OutfitBy},
    title::Title,
};
use crate::census::census_get;
use async_graphql::{ComplexObject, OneofObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

/// Character
/// Source: https://census.daybreakgames.com/get/ps2:v2/character
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct Character {
    #[graphql(name = "id")]
    pub character_id: String,
    pub name: Name,
    pub head_id: String,
    pub certs: Certs,
    pub battle_rank: BattleRank,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub prestige_level: u8,

    /// title_id resolves to Title
    #[graphql(skip)]
    title_id: String,

    /// faction_id resolves to Faction
    #[graphql(skip)]
    faction_id: String,

    #[graphql(skip)]
    outfit: PartialCharacterOutfit,
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
            field,
            value,
            Some(vec!["outfit(outfit_id)"]),
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
    async fn title(&self) -> Title {
        Title::query(self.title_id.clone()).await.unwrap()
    }
    async fn faction(&self) -> Faction {
        Faction::query(self.faction_id.clone()).await.unwrap()
    }
    async fn outfit(&self) -> Option<Outfit> {
        Outfit::query(OutfitBy::Id(self.outfit.outfit_id.clone()))
            .await
            .ok()
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
