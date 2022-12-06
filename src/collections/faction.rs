use crate::prelude::*;

/// Faction (NC, TR, VS, NSO)
/// Source: https://census.lithafalcon.cc/get/ps2/faction
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct Faction {
    pub faction_id: u8,
    #[graphql(skip)]
    pub name: TranslatedString,
    pub code_tag: String,
    pub user_selectable: bool,
}

#[ComplexObject]
impl Faction {
    /// Name with translation support. Query as `name(lang: "ru")`
    async fn name(&self, #[graphql(default = "en")] lang: String) -> String {
        self.name.lang(lang)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct FactionResponse {
    faction_list: Vec<Faction>,
}

impl Faction {
    pub async fn query(id: String) -> Result<Faction, String> {
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
    async fn faction(&self, id: String) -> Faction {
        Faction::query(id).await.unwrap()
    }
}
