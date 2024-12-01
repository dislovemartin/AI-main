use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

#[async_trait]
pub trait Cache {
    async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T>;
    async fn set<T: Serialize>(&self, key: &str, value: T, ttl: Duration) -> bool;
    async fn delete(&self, key: &str) -> bool;
}

pub struct RedisCache {
    client: Client,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> anyhow::Result<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self { client })
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = self.client.get_async_connection().await.ok()?;
        let data: Option<String> = conn.get(key).await.ok()?;
        data.and_then(|d| serde_json::from_str(&d).ok())
    }

    async fn set<T: Serialize>(&self, key: &str, value: T, ttl: Duration) -> bool {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            let serialized = serde_json::to_string(&value).ok()?;
            conn.set_ex(key, serialized, ttl.as_secs() as usize)
                .await
                .is_ok()
        } else {
            false
        }
    }

    async fn delete(&self, key: &str) -> bool {
        if let Ok(mut conn) = self.client.get_async_connection().await {
            conn.del(key).await.is_ok()
        } else {
            false
        }
    }
}
