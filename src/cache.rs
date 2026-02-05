use crate::connectors::ReturnedData;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const TTL_SECONDS: u64 = 60 * 300; // 300 minutes

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CacheEntry<T> {
    pub cached_at: u64,
    pub data: T,
}

type WeatherCache = HashMap<String, CacheEntry<ReturnedData>>;

enum CachePolicy {
    CacheHit(WeatherCache),
    CacheExpired(WeatherCache),
    CacheMiss(WeatherCache),
}

pub fn cache_path() -> PathBuf {
    let mut p = cache_dir().expect("no cache dir");
    p.push("weather_cli");
    fs::create_dir_all(&p).ok();
    p.push("weather.json");
    p
}

pub fn load_cache() -> WeatherCache {
    fs::read_to_string(cache_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default() // remove semicolon
}

pub fn save_cache(cache: &WeatherCache) {
    if let Ok(json) = serde_json::to_string_pretty(cache) {
        let _ = fs::write(cache_path(), json);
    }
}

pub fn clear_cache() {
    let path = cache_path();
    if path.exists() {
        let _ = fs::remove_file(path); // delete file
    }
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn is_valid(entry: &CacheEntry<ReturnedData>) -> bool {
    now() - entry.cached_at < TTL_SECONDS
}

// Fixed: generic insert_save
pub fn insert_save(key: String, data: ReturnedData, cache_file: &mut WeatherCache) {
    cache_file.insert(
        key,
        CacheEntry {
            cached_at: now(),
            data,
        },
    );
    save_cache(&cache_file);
}
