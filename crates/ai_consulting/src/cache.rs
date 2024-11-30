use dashmap::DashMap;
use std::time::{SystemTime, Duration};
use tokio::time::sleep;

pub struct CacheEntry {
    value: ApiResponse,
    expiration: SystemTime,
}

pub struct AppCache {
    cache: DashMap<String, CacheEntry>,
}

impl AppCache {
    pub fn new() -> Self {
        let cache = DashMap::new();
        let cache_clone = cache.clone();
        tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(60)).await;
                let now = SystemTime::now();
                cache_clone.retain(|_, entry| entry.expiration > now);
            }
        });
        AppCache { cache }
    }

    pub fn set(&self, key: String, value: ApiResponse) {
        let expiration = SystemTime::now() + Duration::from_secs(300); // 5 minutes
        self.cache.insert(key, CacheEntry { value, expiration });
    }

    pub fn get(&self, key: &str) -> Option<ApiResponse> {
        self.cache.get(key).map(|entry| entry.value.clone())
    }
}
