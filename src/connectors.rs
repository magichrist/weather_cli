use crate::response_layouts::{Location, WeatherDaily, WeatherResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize)]
/// Expected Data Type from Requests from weather endpoints.
pub enum ReturnedData {
    /// Daily uses WeatherDaily Object.
    Daily(Box<WeatherDaily>),
    /// Current uses WeatherResponse Object.
    Current(Box<WeatherResponse>),
    /// MLocation uses Location Object.
    MLocation(Box<Location>),
}

impl ReturnedData {
    /// Convert and return data to daily
    pub fn daily(&self) -> Option<&WeatherDaily> {
        if let Self::Daily(daily) = self {
            Some(daily)
        } else {
            None
        }
    }

    /// Convert and return data to current
    pub fn current(&self) -> Option<&WeatherResponse> {
        if let Self::Current(current) = self {
            Some(current)
        } else {
            None
        }
    }

    /// Convert and return data to location
    pub fn mlocation(&self) -> Option<&Location> {
        if let Self::MLocation(location) = self {
            Some(location)
        } else {
            None
        }
    }
}

/// Will struct url using given user inputs
pub fn transform_url(api_hook: &str, lat_lon: &[f32]) -> String {
    let mut api_hook = api_hook.replace("LAT", &lat_lon[0].to_string());
    api_hook = api_hook.replace("LON", &lat_lon[1].to_string());
    api_hook
}

/// Will act and call the structured url.
pub async fn fetch(api_hook: String) -> Result<ReturnedData, Box<dyn std::error::Error>> {
    let res = reqwest::get(&api_hook).await?.text().await?;

    // Try parsing as Current
    if let Ok(current) = serde_json::from_str::<WeatherResponse>(&res) {
        return Ok(ReturnedData::Current(Box::from(current)));
    }

    // Try parsing as Daily
    if let Ok(daily) = serde_json::from_str::<WeatherDaily>(&res) {
        return Ok(ReturnedData::Daily(Box::from(daily)));
    }
    // Try parsing as Location
    if let Ok(location) = serde_json::from_str::<Location>(&res) {
        return Ok(ReturnedData::MLocation(Box::from(location)));
    }

    // If neither worked, return a proper error
    Err(format!(
        "Failed to parse response as WeatherResponse or WeatherDaily: {}",
        res
    )
    .into())
}
