use async_graphql::{Object, OneofObject, SimpleObject};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{census::census_get, query};

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
        let query = match by {
            OutfitBy::Id(id) => query!("outfit_id", id),
            OutfitBy::Alias(alias) => query!("alias_lower", alias.to_lowercase()),
            OutfitBy::Name(name) => query!("name_lower", name.to_lowercase()),
        };

        let response = census_get::<OutfitResponse>("outfit", query, None)
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

#[derive(Default)]
pub struct OutfitQuery;

#[Object]
impl OutfitQuery {
    async fn outfit(&self, by: OutfitBy) -> Outfit {
        Outfit::query(by).await.unwrap()
    }
}
