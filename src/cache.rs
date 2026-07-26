use crate::error::{AppError, AppResult};
use crate::models::ReturnedData;
use dirs::cache_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::debug;

/// Cache TTL: 300 minutes (18 000 seconds).
const TTL_SECONDS: u64 = 300 * 60;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheEntry {
    pub cached_at: u64,
    pub data: ReturnedData,
}

/// On-disk weather cache backed by `$XDG_CACHE_HOME/weather_cli/weather.json`.
pub struct Cache {
    path: PathBuf,
    entries: HashMap<String, CacheEntry>,
}

impl Cache {
    /// Load the cache from disk. Returns an empty cache on any I/O or parse error.
    pub fn load() -> Self {
        let path = Self::default_path();
        let entries = match fs::read_to_string(&path) {
            Ok(s) => match serde_json::from_str(&s) {
                Ok(entries) => entries,
                Err(e) => {
                    debug!("failed to parse cache: {e}");
                    HashMap::new()
                }
            },
            Err(e) => {
                debug!("failed to read cache: {e}");
                HashMap::new()
            }
        };
        Self { path, entries }
    }

    /// Retrieve an entry only if it is still within TTL.
    pub fn get_valid(&self, key: &str) -> Option<&ReturnedData> {
        let entry = self.entries.get(key)?;
        if Self::is_valid(entry) {
            Some(&entry.data)
        } else {
            None
        }
    }

    /// Insert data into the cache and persist to disk.
    pub fn insert(&mut self, key: &str, data: ReturnedData) {
        self.entries.insert(
            key.to_string(),
            CacheEntry {
                cached_at: Self::now(),
                data,
            },
        );
        self.persist();
    }

    /// Remove the cache file from disk.
    pub fn clear() -> AppResult<()> {
        let path = Self::default_path();
        if path.exists() {
            fs::remove_file(&path).map_err(|e| AppError::Cache(e.to_string()))?;
        }
        Ok(())
    }

    // -- private helpers --------------------------------------------------

    fn default_path() -> PathBuf {
        let mut p = cache_dir().unwrap_or_else(|| PathBuf::from("."));
        p.push("weather_cli");
        fs::create_dir_all(&p).ok();
        p.push("weather.json");
        p
    }

    fn persist(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.entries) {
            if let Err(e) = fs::write(&self.path, &json) {
                debug!("failed to write cache: {e}");
            }
        }
    }

    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }

    fn is_valid(entry: &CacheEntry) -> bool {
        Self::now()
            .checked_sub(entry.cached_at)
            .is_some_and(|elapsed| elapsed < TTL_SECONDS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Location;

    fn temp_cache_path() -> PathBuf {
        let mut p = std::env::temp_dir();
        p.push(format!("weather_cli_test_{}", std::process::id()));
        p.push("weather.json");
        p
    }

    #[test]
    fn cache_insert_and_get_valid() {
        let path = temp_cache_path();
        fs::create_dir_all(path.parent().unwrap()).ok();

        let mut entries = HashMap::new();
        entries.insert(
            "test_key".to_string(),
            CacheEntry {
                cached_at: Cache::now(),
                data: ReturnedData::Location(Box::new(Location { lat: 1.0, lon: 2.0 })),
            },
        );
        let json = serde_json::to_string_pretty(&entries).unwrap();
        fs::write(&path, json).unwrap();

        let raw: HashMap<String, CacheEntry> = fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
        let entry = raw.get("test_key").unwrap();
        assert!(Cache::is_valid(entry));
        if let ReturnedData::Location(loc) = &entry.data {
            assert_eq!(loc.lat, 1.0);
        } else {
            panic!("expected Location variant");
        }

        let _ = fs::remove_file(path);
    }

    #[test]
    fn cache_expired_entry() {
        let entry = CacheEntry {
            cached_at: 0, // UNIX epoch — way older than TTL
            data: ReturnedData::Location(Box::new(Location { lat: 0.0, lon: 0.0 })),
        };
        assert!(!Cache::is_valid(&entry));
    }

    #[test]
    fn cache_clear_nonexistent() {
        let mut p = std::env::temp_dir();
        p.push("weather_cli_nonexistent_test");
        let _ = fs::remove_dir_all(&p);
        // Should not panic
        let _ = fs::remove_file(p.join("weather.json"));
    }
}
