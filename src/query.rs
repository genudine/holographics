use crate::{collections::character::Character, health::Health};
use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    /// Reports on the health of Genudine Holographics
    async fn health(&self) -> Health {
        Health {}
    }

    /// Returns a graph for the character with the given ID.
    async fn character(
        &self,
        id: Option<String>,
        name: Option<String>,
    ) -> Result<Character, String> {
        Character::query(id, name).await
    }
}
