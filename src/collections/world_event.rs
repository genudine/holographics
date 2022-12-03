// use async_graphql::{ComplexObject, InputObject, SimpleObject};
// use serde::{Deserialize, Serialize};
// use serde_aux::prelude::*;

// use crate::census::census_get;

// #[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
// #[graphql(complex)]
// pub struct WorldEvent {
//     /// Usually MetagameEvent or FacilityControl.
//     pub event_type: String,

//     // MetagameEvent fields
//     pub metagame_event_id: String,
//     pub metagame_event_state: String,

//     /// If you don't care about doing the math, use the instance graph path instance.
//     ///
//     /// Combined 32 bit ID: the top 16 bits are the zone instance id and the bottom 16 bits are the actual zone id.
//     /// Non-instanced/permanent zones will always have 0 in the top 16 bits
//     /// Zone instance IDs start at 1 when the server restarts and increment every time a new instance of that continent is created
//     /// It embeds zone ID, so you'd like 65546 ((1 << 16) | 4) for Hossin instance 1
//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub instance_id: u32,

//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub faction_nc: f32,
//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub faction_tr: f32,
//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub faction_vs: f32,

//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub experience_bonus: u32,

//     #[serde(deserialize_with = "deserialize_number_from_string")]
//     pub timestamp: u64, // TODO: convert to DateTime

//     // Graph mounts
//     #[graphql(skip)]
//     zone_id: String,

//     #[graphql(skip)]
//     world_id: String,
// }

// #[ComplexObject]
// impl WorldEventMetagame {
//     async fn instance(&self) -> Instance {
//         Instance::from(self.instance_id)
//     }
// }

// #[derive(InputObject)]
// pub struct WorldEventParams {
//     pub id: Option<String>,
//     #[graphql(name = "type")]
//     pub event_type: Option<String>,
//     pub world_id: Option<String>,
// }

// impl WorldEvent {
//     pub async fn query(_params: Option<WorldEventParams>) -> Result<Vec<WorldEvent>, String> {
//         let events = census_get::<WorldEventResponse>("world_event", None, None)
//             .await
//             .unwrap()
//             .world_event_list;

//         Ok(events)
//     }
// }

// #[derive(SimpleObject, Serialize, Deserialize, Debug, Clone)]
// struct Instance {
//     /// Top 16 bits of the instance ID, which is the continent.
//     /// If you need this as a graph object, use the parent.
//     pub zone: u32,

//     /// Bottom 16 bits of the instance ID, which is a server-lifetime incrementing ID. This counter resets on server restart.
//     pub id: u32,

//     /// Mirror of the parent graph's instance_id
//     pub raw: u32,
// }

// impl Instance {
//     fn from(instance_id: u32) -> Self {
//         let zone = instance_id >> 16;
//         let id = instance_id & 0xFFFF;
//         Self {
//             zone,
//             id,
//             raw: instance_id,
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct WorldEventResponse {
//     world_event_list: Vec<WorldEvent>,
// }
