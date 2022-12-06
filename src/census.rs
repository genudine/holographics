use crate::cache::Cache;
use async_graphql::Result;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};

lazy_static! {
    static ref DEBUG_RESPONSE: bool =
        std::env::var("DEBUG_RESPONSE").unwrap_or("false".to_string()) == "true";
}

async fn generic_get<RV: DeserializeOwned + Serialize>(
    base_url: &str,
    collection: &'static str,
    query: IndexMap<&'static str, String>,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    let url = format!(
        "{}/{}?{}",
        base_url,
        collection,
        query
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<String>>()
            .join("&")
    );
    println!("URL: {}", url);

    let cache_key = seahash::hash(url.as_bytes()).to_string();

    if let Ok(data) = Cache::get()
        .await
        .get::<String, String>(cache_key.clone())
        .await
    {
        println!(
            "HIT => generic_get([{}] {}, {}, {:#?})",
            cache_key, base_url, collection, query
        );
        return Ok(serde_json::from_str(&data).unwrap());
    }
    println!(
        "MISS => generic_get([{}] {}, {}, {:#?})",
        cache_key.clone(),
        base_url,
        collection,
        query
    );
    // fetch data then cache it
    let resp = reqwest::get(url).await.unwrap();
    let status: u16 = resp.status().into();

    let data = if *DEBUG_RESPONSE {
        let text = resp.text().await.unwrap();
        println!("RESPONSE => {:?} {:?}", status, text);
        match serde_json::from_str::<RV>(&text) {
            Ok(data) => Ok::<RV, serde_json::Error>(data),
            Err(e) => {
                let _: () = Cache::get().await.del(cache_key.clone()).await.unwrap();
                Err(e.to_string())?
            }
        }
    } else {
        match resp.json::<RV>().await {
            Err(e) => {
                let _: () = Cache::get().await.del(cache_key.clone()).await.unwrap();
                Err(e.to_string())?
            }
            Ok(data) => Ok(data),
        }
    }
    .unwrap();

    Cache::get()
        .await
        .set_ex::<String, String, String>(
            cache_key.clone(),
            serde_json::to_string(&data).unwrap(),
            cache_ttl.unwrap_or(60 * 60 * 24),
        )
        .await
        .unwrap();

    Ok(data)
}

/// Get data from the Census API
///
/// Consider setting the cache_ttl to `604800` (7 days) for static data,
/// and `86400` (1 day) for dynamic data. Census is slow, help it out.
/// Defaults to 1 day.
///
/// Knows of a special query item:
///   - `c:platform`:`ps2|ps2ps4us|ps2ps4eu` (default: `ps2`): Which platform should be queried
pub async fn census_get<RV: DeserializeOwned + Serialize>(
    collection: &'static str,
    mut query: IndexMap<&'static str, String>,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    let platform = match query.remove("c:platform") {
        Some(platform) => platform,
        None => "ps2".to_string(),
    };

    generic_get(
        format!("https://census.daybreakgames.com/s:saegd/get/{}", platform).as_str(),
        collection,
        query,
        match cache_ttl {
            Some(ttl) => Some(ttl),
            None => Some(60 * 60 * 24), // 1 day
        },
    )
    .await
}

/// Get data from the Sanctuary API
///
/// Cache TTL defaults to 7 days.
///
/// Knows of two special query items:
///   - `c:censusJSON`: `true|false` (default: `false`): whether to return the raw census JSON or type-corrected data
///   - `c:platform`: `ps2|pts` (default: `ps2`): PS2 PC Live or PTS data
pub async fn sanctuary_get<RV: DeserializeOwned + Serialize>(
    collection: &'static str,
    mut query: IndexMap<&'static str, String>,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    if !query.contains_key("c:censusJSON") {
        query.insert("c:censusJSON", "false".to_string());
    }

    let platform = match query.remove("c:platform") {
        Some(platform) => platform,
        None => "ps2".to_string(),
    };

    generic_get(
        format!("https://census.lithafalcon.cc/get/{}", platform).as_str(),
        collection,
        query,
        match cache_ttl {
            Some(ttl) => Some(ttl),
            None => Some(60 * 60 * 24 * 7), // 1 week, as Sanctuary is mostly static data
        },
    )
    .await
}
