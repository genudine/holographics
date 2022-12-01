use super::generic_types::TranslatedString;
use crate::census::sanctuary_get;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

/// Faction (NC, TR, VS, NSO)
/// Source: https://census.lithafalcon.cc/get/ps2/faction
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct Faction {
    pub faction_id: u8,
    pub name: TranslatedString,
    pub code_tag: String,
    pub user_selectable: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FactionResponse {
    faction_list: Vec<Faction>,
}

impl Faction {
    pub async fn query(id: String) -> Result<Faction, String> {
        sanctuary_get::<FactionResponse>("faction", "faction_id", id, None)
            .await
            .unwrap()
            .faction_list
            .pop()
            .ok_or("No faction found".to_string())
    }
}
