use crate::cache::{self, Cache};
use async_graphql::Result;
use lazy_static::lazy_static;
use redis::AsyncCommands;
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
    resolves: Option<Vec<&'static str>>,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    let cache_key = format!("{}-{}:{}:{}", cache_prefix, collection, field, value);
    if let Ok(data) = Cache::get()
        .await
        .get::<String, String>(cache_key.clone())
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
        "{}/{}/?c:censusJSON=false&{}={}{}",
        base_url,
        collection,
        field,
        value,
        if resolves.is_some() {
            format!("&c:resolve={}", resolves.unwrap().join(","))
        } else {
            "".to_string()
        }
    ))
    .await
    .unwrap();

    let data = if *DEBUG_RESPONSE {
        let text = resp.text().await.unwrap();
        println!("RESPONSE => {:?}", text);
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
            cache_key,
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
    resolves: Option<Vec<&'static str>>,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    generic_get(
        "https://census.daybreakgames.com/s:saegd/get/ps2:v2",
        "census",
        collection,
        field,
        value,
        resolves,
        cache_ttl,
    )
    .await
}

pub async fn sanctuary_get<RV: DeserializeOwned + Serialize>(
    collection: &'static str,
    field: &'static str,
    value: String,
    resolves: Option<Vec<&'static str>>,
    cache_ttl: Option<usize>,
) -> Result<RV> {
    generic_get(
        "https://census.lithafalcon.cc/get/ps2",
        "sanctu",
        collection,
        field,
        value,
        resolves,
        cache_ttl,
    )
    .await
}
