use serde::{Deserialize, Serialize};
use crate::response_layouts::{Location, WeatherDaily, WeatherResponse};
use reqwest;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum ReturnedData {
    Daily(WeatherDaily),
    Current(WeatherResponse),
    MLocation(Location),
}

impl ReturnedData {
    pub fn daily(&self) -> Option<&WeatherDaily> {
        if let Self::Daily(daily) = self {Some(daily)}else {None}
    }
    pub fn current(&self) -> Option<&WeatherResponse> {
        if let Self::Current(current) = self {Some(current)}else {None}
    }
    pub fn mlocation(&self) -> Option<&Location> {
        if let Self::MLocation(location) = self {Some(location)}else {None}
    }
}

pub fn transform_url(api_hook: &str, lat_lon: &[f32]) -> String {
    let mut api_hook = api_hook.replace("LAT", &lat_lon[0].to_string());
    api_hook = api_hook.replace("LON", &lat_lon[1].to_string());
    api_hook
}

pub async fn fetch(api_hook: String) -> Result<ReturnedData, Box<dyn std::error::Error>> {
    let res = reqwest::get(&api_hook).await?.text().await?;

    // Try parsing as Current first
    if let Ok(current) = serde_json::from_str::<WeatherResponse>(&res) {
        return Ok(ReturnedData::Current(current));
    }

    // Then try Daily
    if let Ok(daily) = serde_json::from_str::<WeatherDaily>(&res) {
        return Ok(ReturnedData::Daily(daily));
    }

    if let Ok(location) = serde_json::from_str::<Location>(&res) {
        return Ok(ReturnedData::MLocation(location));
    }

    // If neither worked, return a proper error
    Err(format!("Failed to parse response as WeatherResponse or WeatherDaily: {}", res).into())
}
