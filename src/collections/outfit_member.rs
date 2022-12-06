use super::{
    character::{Character, CharacterBy},
    outfit::{Outfit, OutfitBy},
};
use crate::prelude::*;

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct OutfitMember {
    #[serde(default)]
    pub outfit_id: String,
    pub character_id: String,
    // pub member_since_date: String,
    // #[serde(deserialize_with = "deserialize_number_from_string")]
    // pub member_since: u64,
    pub rank: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub rank_ordinal: u8,
}

impl OutfitMember {
    // async fn query(outfit_id: String) -> Result<OutfitMember, String> {
    //     // let mut query = query!("outfit_id", outfit_id);
    //     // query.insert("c:resolve", "character_id".to_string());

    //     // let response = census_get::<OutfitMemberResponse>("outfit_member", query).await?;

    //     // Ok(response.outfit_member_list)
    // }
}

#[ComplexObject]
impl OutfitMember {
    async fn character(&self) -> Character {
        Character::query(CharacterBy::Id(self.character_id.clone()))
            .await
            .unwrap()
    }

    async fn outfit(&self) -> Option<Outfit> {
        if self.outfit_id.is_empty() {
            None
        } else {
            Some(
                Outfit::query(OutfitBy::Id(self.outfit_id.clone()))
                    .await
                    .unwrap(),
            )
        }
    }
}
