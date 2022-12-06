use crate::prelude::*;

/// World as in Connery, Emerald, etc.
/// Source: https://census.lithafalcon.cc/get/ps2/world
/// Source: https://census.daybreakgames.com/get/ps2/world
#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct World {
    /// Either `census` or `sanctuary`, depending on the source selected. If this is empty, assume `sanctuary`.
    #[serde(skip)]
    pub source: &'static str,

    #[graphql(skip)]
    pub name: TranslatedString,

    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub world_id: String,

    /// **SANCTUARY ONLY**: Can players log in to this world?
    #[serde(
        default,
        deserialize_with = "deserialize_bool_from_anything",
        alias = "is_locked"
    )]
    pub is_locked: bool,
    /// **SANCTUARY ONLY**: Can players make characters on this world?
    #[serde(
        default,
        deserialize_with = "deserialize_bool_from_anything",
        alias = "is_unprivileged_access_allowed"
    )]
    pub is_unprivileged_access_allowed: bool,

    /// **CENSUS ONLY**: An inaccurate representation of server "online" state.
    #[serde(default)]
    pub state: String,
}

#[ComplexObject]
impl World {
    /// Name with translation support. Query as `name(lang: "de")`
    async fn name(&self, #[graphql(default = "en")] lang: String) -> String {
        self.name.lang(lang)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct WorldResponse {
    world_list: Vec<World>,
}

impl World {
    pub async fn query(id: String, use_census: bool) -> Result<World, String> {
        let response = if use_census {
            census_get::<WorldResponse>("world", query!("world_id", id.clone()), None).await
        } else {
            let mut query = query!("world_id", id.clone());
            query.insert("c:censusJSON", "true".to_string());
            sanctuary_get::<WorldResponse>("world", query, None).await
        };

        let world = response
            .unwrap()
            .world_list
            .pop()
            .ok_or("No world found".to_string());

        match world {
            Ok(mut world) => {
                world.source = if use_census { "census" } else { "sanctuary" };
                Ok(world)
            }
            Err(e) => Err(e),
        }
    }
}

#[derive(Default)]
pub struct WorldQuery;

#[Object]
impl WorldQuery {
    async fn world(
        &self,
        id: String,
        #[graphql(default = false)] use_census: bool,
    ) -> Result<World, String> {
        World::query(id, use_census).await
    }
}
