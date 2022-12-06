use crate::prelude::*;

/// Title
/// Source: https://census.daybreakgames.com/get/ps2/title
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct Title {
    #[graphql(name = "id")]
    pub title_id: String,

    #[graphql(skip)]
    pub name: TranslatedString,
}

#[ComplexObject]
impl Title {
    async fn name(&self, #[graphql(default = "en")] lang: String) -> String {
        self.name.lang(lang)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TitleResponse {
    title_list: Vec<Title>,
}

impl Title {
    pub async fn query(id: String) -> Result<Title, String> {
        census_get::<TitleResponse>("title", query!("title_id", id), None)
            .await
            .unwrap()
            .title_list
            .pop()
            .ok_or("No title found".to_string())
    }
}

#[derive(Default)]
pub struct TitleQuery;

#[Object]
impl TitleQuery {
    async fn title(&self, id: String) -> Title {
        Title::query(id).await.unwrap()
    }
}
