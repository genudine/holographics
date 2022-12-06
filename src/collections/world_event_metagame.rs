use super::{world::World, zone::Zone};
use crate::prelude::*;

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(complex)]
pub struct WorldEventMetagame {
    /// Always MetagameEvent
    pub event_type: String,

    // MetagameEvent fields
    pub metagame_event_id: String,
    pub metagame_event_state: String,

    /// If you don't care about doing the math, use the instance graph path instance.
    ///
    /// Combined 32 bit ID: the top 16 bits are the zone instance id and the bottom 16 bits are the actual zone id.
    /// Non-instanced/permanent zones will always have 0 in the top 16 bits
    /// Zone instance IDs start at 1 when the server restarts and increment every time a new instance of that continent is created
    /// It embeds zone ID, so you'd like 65546 ((1 << 16) | 4) for Hossin instance 1
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub instance_id: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub faction_nc: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub faction_tr: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub faction_vs: f32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub experience_bonus: u32,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub timestamp: u64, // TODO: convert to DateTime

    // Graph mounts
    #[graphql(skip)]
    zone_id: String,
    #[graphql(skip)]
    world_id: String,
}

impl WorldEventMetagame {
    pub async fn query(world_id: Option<Vec<String>>) -> Result<Vec<WorldEventMetagame>, String> {
        let mut query = match world_id {
            Some(world_id) => query!("world_id", world_id.join(",")),
            None => query!(),
        };
        query.insert("type", "METAGAME".to_string());

        let response =
            census_get::<WorldEventResponse<WorldEventMetagame>>("world_event", query, None)
                .await
                .unwrap()
                .world_event_list;

        Ok(response)
    }
}

#[ComplexObject]
impl WorldEventMetagame {
    async fn instance(&self) -> Instance {
        Instance::from(self.instance_id)
    }

    async fn world(&self) -> World {
        World::query(self.world_id.clone(), false).await.unwrap()
    }

    async fn zone(&self) -> Zone {
        Zone::query(self.zone_id.clone()).await.unwrap()
    }
}

#[derive(Default)]
pub struct WorldEventMetagameQuery;

#[Object]
impl WorldEventMetagameQuery {
    /// world_events with type = MetagameEvent. Types are hard. forgive me.
    async fn world_event_metagame(&self, world_id: Option<Vec<String>>) -> Vec<WorldEventMetagame> {
        WorldEventMetagame::query(world_id).await.unwrap()
    }
}

#[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
struct Instance {
    /// Top 16 bits of the instance ID, which is the continent.
    /// If you need this as a graph object, use the parent.
    pub zone: u32,

    /// Bottom 16 bits of the instance ID, which is a server-lifetime incrementing ID. This counter resets on server restart.
    pub id: u32,

    /// Mirror of the parent graph's instance_id
    pub raw: u32,
}

impl Instance {
    fn from(instance_id: u32) -> Self {
        let zone = instance_id >> 16;
        let id = instance_id & 0xFFFF;
        Self {
            zone,
            id,
            raw: instance_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WorldEventResponse<WET> {
    world_event_list: Vec<WET>,
}
