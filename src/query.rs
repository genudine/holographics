use crate::{
    collections::{
        character::CharacterQuery, faction::FactionQuery, outfit::OutfitQuery, title::TitleQuery,
        world::WorldQuery, world_event_metagame::WorldEventMetagameQuery, zone::ZoneQuery,
    },
    health::HealthQuery,
};
use async_graphql::MergedObject;

/// Start here. This is the root of the GraphQL API.
#[derive(MergedObject, Default)]
pub struct Query(
    // Please maintain alphabetical order
    CharacterQuery,
    FactionQuery,
    HealthQuery,
    OutfitQuery,
    TitleQuery,
    WorldEventMetagameQuery,
    WorldQuery,
    ZoneQuery,
);
