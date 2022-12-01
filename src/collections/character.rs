use super::{faction::Faction, title::Title};
use crate::census::census_get;
use async_graphql::{ComplexObject, SimpleObject};
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CharacterResponse {
    character_list: Vec<Character>,
}

impl Character {
    pub async fn query(id: Option<String>, name: Option<String>) -> Result<Character, String> {
        let (field, value) = match (id, name) {
            (Some(id), None) => ("character_id", id),
            (None, Some(name)) => ("name.first_lower", name.to_lowercase()),
            _ => return Err("Must provide either an ID or a name, and not both.".to_string()),
        };

        let response = census_get::<CharacterResponse>("character", field, value, None)
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
