use crate::cache::Cache;
use lazy_static::lazy_static;
use redis::AsyncCommands;
use reqwest::Result;
use serde::{de::DeserializeOwned, Serialize};

lazy_static! {
    static ref DEBUG_RESPONSE: bool =
        std::env::var("DEBUG_RESPONSE").unwrap_or("false".to_string()) == "true";
}

async fn generic_get<RV: DeserializeOwned + Serialize>(
    base_url: &'static str,
    cache_prefix: &'static str,
    collection: &'static str,
    field: &'static str,
    value: String,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    if let Ok(data) = Cache::get()
        .await
        .get::<String, String>(format!(
            "{}-{}:{}:{}",
            cache_prefix, collection, field, value
        ))
        .await
    {
        println!("HIT => generic_get({}, {}, {})", cache_prefix, field, value);
        return Ok(serde_json::from_str(&data).unwrap());
    }
    println!(
        "MISS => generic_get({}, {}, {})",
        cache_prefix, field, value
    );
    // fetch data then cache it
    let resp = reqwest::get(format!(
        "{}/{}/?c:censusJSON=false&{}={}",
        base_url, collection, field, value
    ))
    .await
    .unwrap();

    let data = if *DEBUG_RESPONSE {
        let text = resp.text().await.unwrap();
        println!("RESPONSE => {:?}", text);
        serde_json::from_str(&text).unwrap()
    } else {
        resp.json().await.unwrap()
    };

    Cache::get()
        .await
        .set_ex::<String, String, String>(
            format!("{}-{}:{}:{}", cache_prefix, collection, field, value),
            serde_json::to_string(&data).unwrap(),
            cache_ttl.unwrap_or(60 * 60 * 24),
        )
        .await
        .unwrap();

    Ok(data)
}

pub async fn census_get<RV: DeserializeOwned + Serialize>(
    collection: &'static str,
    field: &'static str,
    value: String,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    generic_get(
        "https://census.daybreakgames.com/s:saegd/get/ps2:v2",
        "census",
        collection,
        field,
        value,
        cache_ttl,
    )
    .await
}

pub async fn sanctuary_get<RV: DeserializeOwned + Serialize>(
    collection: &'static str,
    field: &'static str,
    value: String,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    generic_get(
        "https://census.lithafalcon.cc/get/ps2",
        "sanctu",
        collection,
        field,
        value,
        cache_ttl,
    )
    .await
}
