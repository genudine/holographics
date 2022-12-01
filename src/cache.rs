use async_once::AsyncOnce;
use lazy_static::lazy_static;

lazy_static! {
    static ref REDIS: AsyncOnce<redis::aio::MultiplexedConnection> = AsyncOnce::new(async {
        let redis_url = format!(
            "redis://{}:{}",
            std::env::var("REDIS_HOST").unwrap_or("localhost".to_string()),
            std::env::var("REDIS_PORT").unwrap_or("6379".to_string()),
        );

        redis::Client::open(redis_url)
            .unwrap()
            .get_multiplexed_tokio_connection()
            .await
            .unwrap()
    });
}

pub struct Cache {}

impl Cache {
    pub async fn get() -> redis::aio::MultiplexedConnection {
        REDIS.get().await.to_owned()
    }
}
