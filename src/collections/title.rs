use crate::census::census_get;

use super::generic_types::TranslatedString;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

/// Title
/// Source: https://census.daybreakgames.com/get/ps2/title
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
pub struct Title {
    #[graphql(name = "id")]
    pub title_id: String,
    pub name: TranslatedString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TitleResponse {
    title_list: Vec<Title>,
}

impl Title {
    pub async fn query(id: String) -> Result<Title, String> {
        census_get::<TitleResponse>("title", "title_id", id, None)
            .await
            .unwrap()
            .title_list
            .pop()
            .ok_or("No title found".to_string())
    }
}
