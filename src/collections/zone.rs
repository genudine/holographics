use super::generic::TranslatedString;
use crate::{census::sanctuary_get, query};
use async_graphql::{ComplexObject, Object, SimpleObject};
use serde::{Deserialize, Serialize};

/// Zone (Indar, Hossin...)
/// Source: https://census.lithafalcon.cc/get/ps2/zone
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct Zone {
    pub zone_id: u32,
    pub hex_size: u16,

    /// Technically, this is `name.en`.
    pub code: String,

    #[graphql(skip)]
    pub name: TranslatedString,
    #[graphql(skip)]
    pub description: TranslatedString,
}

#[ComplexObject]
impl Zone {
    /// Name with translation support. Default is `en`. Query as `name(lang: "ru")`
    async fn name(&self, #[graphql(default = "en")] lang: String) -> String {
        self.name.lang(lang)
    }

    /// Description with translation support. Default is `en`. Query as `description(lang: "ru")`
    async fn description(&self, #[graphql(default = "en")] lang: String) -> String {
        self.description.lang(lang)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FactionResponse {
    faction_list: Vec<Zone>,
}

impl Zone {
    pub async fn query(id: String) -> Result<Zone, String> {
        sanctuary_get::<FactionResponse>("faction", query!("faction_id", id.clone()), None)
            .await
            .unwrap()
            .faction_list
            .pop()
            .ok_or("No faction found".to_string())
    }
}

#[derive(Default)]
pub struct FactionQuery;

#[Object]
impl FactionQuery {
    async fn faction(&self, id: String) -> Zone {
        Zone::query(id).await.unwrap()
    }
}
