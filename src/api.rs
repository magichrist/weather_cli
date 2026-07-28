use crate::error::{AppError, AppResult};
use crate::models::{CityResult, Location, WeatherDaily, WeatherHourly, WeatherResponse};
use colored::Colorize;
use rattles::presets::prelude as presets;
use std::io::Write;
use std::time::Duration;

const API_BASE: &str = "https://api.open-meteo.com/v1/forecast";
const IP_API_URL: &str = "http://ip-api.com/json?fields=lat,lon";
const HTTP_TIMEOUT: Duration = Duration::from_secs(15);

/// Build a shared reqwest client with a timeout.
fn http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(HTTP_TIMEOUT)
        .build()
        .expect("failed to build HTTP client")
}

/// Build an Open-Meteo API URL with the given query parameters.
fn build_url(lat: f64, lon: f64, extra: &[(&str, &str)]) -> String {
    let mut url = reqwest::Url::parse(API_BASE).expect("invalid API base URL");
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("latitude", &lat.to_string());
        q.append_pair("longitude", &lon.to_string());
        for (k, v) in extra {
            q.append_pair(k, v);
        }
    }
    url.to_string()
}

/// Fetch raw text from a URL with a terminal spinner.
async fn fetch_raw(url: &str, msg: &str) -> AppResult<String> {
    let rattle = presets::rain();
    let client = http_client();
    let request = tokio::spawn({
        let url = url.to_string();
        async move { client.get(&url).send().await?.text().await }
    });

    while !request.is_finished() {
        print!("\r{} {msg}          ", rattle.current_frame().bright_blue());
        let _ = std::io::stdout().flush();
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    println!("\r");

    Ok(request.await??)
}

/// Fetch current weather for the given coordinates.
pub async fn fetch_current(lat: f64, lon: f64) -> AppResult<WeatherResponse> {
    let url = build_url(
        lat,
        lon,
        &[(
            "current",
            "temperature_2m,apparent_temperature,relative_humidity_2m,surface_pressure,wind_speed_10m,uv_index,weather_code,rain,snowfall,precipitation",
        )],
    );
    let json = fetch_raw(&url, "Fetching current weather...").await?;
    serde_json::from_str(&json).map_err(|_| AppError::UnexpectedResponse)
}

/// Fetch 7-day forecast for the given coordinates.
pub async fn fetch_forecast(lat: f64, lon: f64) -> AppResult<WeatherDaily> {
    let url = build_url(
        lat,
        lon,
        &[
            (
                "daily",
                "uv_index_max,snowfall_sum,showers_sum,rain_sum,shortwave_radiation_sum,temperature_2m_mean,wind_speed_10m_max",
            ),
            ("timezone", "GMT"),
        ],
    );
    let json = fetch_raw(&url, "Fetching forecast...").await?;
    serde_json::from_str(&json).map_err(|_| AppError::UnexpectedResponse)
}

/// Determine the caller's location from their public IP address.
pub async fn fetch_location() -> AppResult<Location> {
    let json = fetch_raw(IP_API_URL, "Determining location...").await?;
    serde_json::from_str(&json).map_err(|_| AppError::UnexpectedResponse)
}

/// Search for cities by name via the Open-Meteo Geocoding API.
pub async fn search_city(name: &str) -> AppResult<Vec<CityResult>> {
    let mut url = reqwest::Url::parse("https://geocoding-api.open-meteo.com/v1/search")
        .expect("invalid geocoding base URL");
    {
        let mut q = url.query_pairs_mut();
        q.append_pair("name", name);
        q.append_pair("count", "5");
        q.append_pair("language", "en");
    }
    let json = fetch_raw(url.as_str(), "Searching cities...").await?;
    #[derive(serde::Deserialize)]
    struct GeocodingResponse {
        results: Option<Vec<CityResult>>,
    }
    let resp: GeocodingResponse =
        serde_json::from_str(&json).map_err(|_| AppError::UnexpectedResponse)?;
    Ok(resp.results.unwrap_or_default())
}

/// Fetch hourly forecast for the given coordinates.
pub async fn fetch_hourly(lat: f64, lon: f64, days: u32) -> AppResult<WeatherHourly> {
    let url = build_url(
        lat,
        lon,
        &[
            (
                "hourly",
                "temperature_2m,precipitation_probability,weather_code",
            ),
            ("timezone", "GMT"),
            ("forecast_days", &days.to_string()),
        ],
    );
    let json = fetch_raw(&url, "Fetching hourly forecast...").await?;
    serde_json::from_str(&json).map_err(|_| AppError::UnexpectedResponse)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_url_basic() {
        let url = build_url(48.8566, 2.3522, &[("current", "temperature_2m")]);
        assert!(url.contains("latitude=48.8566"));
        assert!(url.contains("longitude=2.3522"));
        assert!(url.contains("current=temperature_2m"));
        assert!(url.starts_with("https://api.open-meteo.com/v1/forecast?"));
    }

    #[test]
    fn build_url_negative_coords() {
        let url = build_url(-33.8688, 151.2093, &[]);
        assert!(url.contains("latitude=-33.8688"));
        assert!(url.contains("longitude=151.2093"));
    }

    #[test]
    fn build_url_multiple_params() {
        let url = build_url(
            0.0,
            0.0,
            &[("daily", "temperature_2m_mean"), ("timezone", "GMT")],
        );
        assert!(url.contains("daily=temperature_2m_mean"));
        assert!(url.contains("timezone=GMT"));
    }

    #[test]
    fn build_url_zero_coords() {
        let url = build_url(0.0, 0.0, &[]);
        assert!(url.contains("latitude=0"));
        assert!(url.contains("longitude=0"));
    }
}
