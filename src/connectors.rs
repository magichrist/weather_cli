use crate::response_layouts::{Location, WeatherDaily, WeatherResponse};
use colored::Colorize;
use rattles::presets::prelude as presets;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::time::Duration;

#[derive(Serialize, Debug, Clone, Deserialize)]
pub enum ReturnedData {
    Daily(Box<WeatherDaily>),
    Current(Box<WeatherResponse>),
    MLocation(Box<Location>),
}

impl ReturnedData {
    pub fn as_daily(&self) -> Option<&WeatherDaily> {
        if let Self::Daily(daily) = self {
            Some(daily)
        } else {
            None
        }
    }

    pub fn as_current(&self) -> Option<&WeatherResponse> {
        if let Self::Current(current) = self {
            Some(current)
        } else {
            None
        }
    }

    pub fn as_location(&self) -> Option<&Location> {
        if let Self::MLocation(location) = self {
            Some(location)
        } else {
            None
        }
    }
}

/// Build the API URL by substituting LAT/LON placeholders.
pub fn transform_url(api_hook: &str, lat_lon: &[f32]) -> String {
    let result = api_hook.replace("LAT", &lat_lon[0].to_string());
    result.replace("LON", &lat_lon[1].to_string())
}

/// Fetch weather data from the API with a terminal spinner.
pub async fn fetch(
    api_hook: String,
    msg: &str,
) -> Result<ReturnedData, Box<dyn std::error::Error>> {
    let rattle = presets::rain();

    let request = tokio::spawn(async move { reqwest::get(&api_hook).await?.text().await });

    while !request.is_finished() {
        print!("\r{} {msg}          ", rattle.current_frame().bright_blue());
        let _ = std::io::stdout().flush();
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    println!("\r");

    let res = request.await??;

    if let Ok(current) = serde_json::from_str::<WeatherResponse>(&res) {
        return Ok(ReturnedData::Current(Box::new(current)));
    }

    if let Ok(daily) = serde_json::from_str::<WeatherDaily>(&res) {
        return Ok(ReturnedData::Daily(Box::new(daily)));
    }

    if let Ok(location) = serde_json::from_str::<Location>(&res) {
        return Ok(ReturnedData::MLocation(Box::new(location)));
    }

    Err(format!("Failed to parse API response: {res}").into())
}
