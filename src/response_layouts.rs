use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherDaily {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub daily_units: DailyUnits,
    pub daily: Daily,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub current_units: CurrentUnits,
    pub current: Current,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DailyUnits {
    pub time: String,
    pub uv_index_max: String,
    pub snowfall_sum: String,
    pub showers_sum: String,
    pub rain_sum: String,
    pub shortwave_radiation_sum: String,
    pub temperature_2m_mean: String,
    pub wind_speed_10m_max: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Daily {
    pub time: Vec<String>,
    pub uv_index_max: Vec<f64>,
    pub snowfall_sum: Vec<f64>,
    pub showers_sum: Vec<f64>,
    pub rain_sum: Vec<f64>,
    pub shortwave_radiation_sum: Vec<f64>,
    pub temperature_2m_mean: Vec<f64>,
    pub wind_speed_10m_max: Vec<f64>,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrentUnits {
    pub temperature_2m: String,
    pub wind_speed_10m: String,
    pub rain: String,
    pub snowfall: String,
    pub precipitation: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Current {
    pub time: String,
    pub temperature_2m: f32,
    pub wind_speed_10m: f32,
    pub rain: f32,
    pub snowfall: f32,
    pub precipitation: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    pub lat:f64,
    pub lon:f64,
}

// #[allow(dead_code)]
// #[derive(Debug, Deserialize)]
// pub struct IpLanLot {
//
// }