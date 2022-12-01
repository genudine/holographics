use async_graphql::{OneofObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::census::census_get;

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct Outfit {
    #[graphql(name = "id")]
    outfit_id: String,
    name: String,
    name_lower: String,
    alias: String,
    alias_lower: String,

    #[graphql(skip)]
    leader_character_id: String,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    member_count: u32,
}

#[derive(OneofObject, Debug)]
pub enum OutfitBy {
    Id(String),
    Name(String),
    Alias(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OutfitResponse {
    outfit_list: Vec<Outfit>,
}

impl Outfit {
    pub async fn query(by: OutfitBy) -> Result<Outfit, String> {
        let (field, value) = match by {
            OutfitBy::Id(id) => ("outfit_id", id),
            OutfitBy::Alias(alias) => ("alias_lower", alias.to_lowercase()),
            OutfitBy::Name(name) => ("name_lower", name.to_lowercase()),
        };

        let response = census_get::<OutfitResponse>("outfit", field, value, None, None)
            .await
            .unwrap()
            .outfit_list
            .pop();

        match response {
            Some(outfit) => Ok(outfit),
            None => Err("No outfit found".to_string()),
        }
    }
}
