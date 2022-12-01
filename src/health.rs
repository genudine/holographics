use async_graphql::{Enum, Object};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
enum UpDown {
    /// The service is up and running
    Up,

    /// The service is down
    Down,
}

pub struct Health {}

/// Reports on the health of Genudine Holographics
#[Object]
impl Health {
    /// Does the official API respond to requests?
    async fn daybreak(&self) -> UpDown {
        reqwest::get("https://census.daybreakgames.com/s:saegd/count/ps2:v2/item")
            .await
            .map(|resp| {
                if resp.status().is_success() {
                    UpDown::Up
                } else {
                    UpDown::Down
                }
            })
            .unwrap_or(UpDown::Down)
    }

    /// Does the Sanctuary API respond to requests?
    async fn sanctuary(&self) -> UpDown {
        reqwest::get("https://census.lithafalcon.cc/count/ps2:v2/item")
            .await
            .map(|resp| {
                if resp.status().is_success() {
                    UpDown::Up
                } else {
                    UpDown::Down
                }
            })
            .unwrap_or(UpDown::Down)
    }
}
