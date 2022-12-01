use crate::{
    collections::{
        character::{Character, CharacterBy},
        faction::Faction,
        outfit::{Outfit, OutfitBy},
        title::Title,
    },
    health::Health,
};
use async_graphql::Object;

pub struct Query;

/// Start here. This is the root of the GraphQL API.
#[Object]
impl Query {
    /// Reports on the health of Genudine Holographics
    async fn health(&self) -> Health {
        Health {}
    }

    /// Returns a graph for the character with the given ID or name (case-insensitive).
    /// Example: `character(by: { name: "wrel" })`
    async fn character(&self, by: CharacterBy) -> Result<Character, String> {
        Character::query(by).await
    }

    /// Returns a graph for the faction with the given ID.
    async fn faction(&self, id: String) -> Result<Faction, String> {
        Faction::query(id).await
    }

    /// Returns a graph for the title with the given ID.
    async fn title(&self, id: String) -> Result<Title, String> {
        Title::query(id).await
    }

    /// Returns a graph for the outfit with the given ID or alias/tag (case-insensitive), or name (case-insensitive).
    /// Example: `outfit(by: { id: "1" })`
    async fn outfit(&self, by: OutfitBy) -> Result<Outfit, String> {
        Outfit::query(by).await
    }
}
