use crate::{
    collections::{
        character::CharacterQuery, faction::FactionQuery, outfit::OutfitQuery, title::TitleQuery,
    },
    health::HealthQuery,
};
use async_graphql::MergedObject;

/// Start here. This is the root of the GraphQL API.
#[derive(MergedObject, Default)]
pub struct Query(
    HealthQuery,
    CharacterQuery,
    FactionQuery,
    TitleQuery,
    OutfitQuery,
);
