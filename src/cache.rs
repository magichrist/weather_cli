use crate::connectors::ReturnedData;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// Time To Live: 300 Minutes
const TTL_SECONDS: u64 = 60 * 300; // 300 minutes

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
/// Cache Object
pub struct CacheEntry<T> {
    /// Time in UNIX_EPOCH
    pub cached_at: u64,
    /// Data to cache
    pub data: T,
}
/// Type Handler for HashMap for Cache
type WeatherCache = HashMap<String, CacheEntry<ReturnedData>>;

/// get the path for cache file or create it: weather.json
pub fn cache_path() -> PathBuf {
    let mut p = cache_dir().expect("no cache dir");
    p.push("weather_cli");
    fs::create_dir_all(&p).ok();
    p.push("weather.json");
    p
}

/// read the weather.json
pub fn load_cache() -> WeatherCache {
    fs::read_to_string(cache_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default() // remove semicolon
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
        let _ = fs::remove_file(path); // delete file
    }
}

/// Returns currect time in UNIX Epoch
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Checks validity of cache:
/// NOW - cached_at < TTL
pub fn is_valid(entry: &CacheEntry<ReturnedData>) -> bool {
    now() - entry.cached_at < TTL_SECONDS
}

// Fixed: generic insert_save
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
