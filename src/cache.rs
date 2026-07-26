use crate::connectors::ReturnedData;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Time To Live: 300 minutes (18000 seconds)
const TTL_SECONDS: u64 = 60 * 300;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct CacheEntry<T> {
    /// Time in UNIX_EPOCH
    pub cached_at: u64,
    /// Data to cache
    pub data: T,
}

type WeatherCache = HashMap<String, CacheEntry<ReturnedData>>;

/// Get the path for cache file or create it: weather.json
pub fn cache_path() -> PathBuf {
    let mut p = cache_dir().expect("no cache dir");
    p.push("weather_cli");
    fs::create_dir_all(&p).ok();
    p.push("weather.json");
    p
}

/// Read the weather.json
pub fn load_cache() -> WeatherCache {
    fs::read_to_string(cache_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

/// Writes cache file to disk
pub fn save_cache(cache: &WeatherCache) {
    if let Ok(json) = serde_json::to_string_pretty(cache) {
        let _ = fs::write(cache_path(), json);
    }
}

/// Clears and deletes weather.json file.
pub fn clear_cache() {
    let path = cache_path();
    if path.exists() {
        let _ = fs::remove_file(path);
    }
}

/// Returns current time in UNIX Epoch
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Checks validity of cache: NOW - cached_at < TTL
pub fn is_valid(entry: &CacheEntry<ReturnedData>) -> bool {
    now()
        .checked_sub(entry.cached_at)
        .is_some_and(|elapsed| elapsed < TTL_SECONDS)
}

/// Insert cache into cache file and uses save_cache.
pub fn insert_save(key: String, data: ReturnedData, cache_file: &mut WeatherCache) {
    cache_file.insert(
        key,
        CacheEntry {
            cached_at: now(),
            data,
        },
    );
    save_cache(cache_file);
}
