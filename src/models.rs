use serde::{Deserialize, Serialize};

/// Response from the Open-Meteo forecast daily endpoint.
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

/// Response from the Open-Meteo current weather endpoint.
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CurrentUnits {
    pub temperature_2m: String,
    pub apparent_temperature: String,
    pub relative_humidity_2m: String,
    pub surface_pressure: String,
    pub wind_speed_10m: String,
    pub uv_index: String,
    pub weather_code: String,
    pub rain: String,
    pub snowfall: String,
    pub precipitation: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Current {
    pub time: String,
    pub temperature_2m: f64,
    pub apparent_temperature: f64,
    pub relative_humidity_2m: f64,
    pub surface_pressure: f64,
    pub wind_speed_10m: f64,
    pub uv_index: f64,
    pub weather_code: u32,
    pub rain: f64,
    pub snowfall: f64,
    pub precipitation: f64,
}

/// IP-based geolocation response.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

/// Tagged union for caching heterogeneous API responses.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReturnedData {
    Daily(Box<WeatherDaily>),
    Current(Box<WeatherResponse>),
    Hourly(Box<WeatherHourly>),
    Location(Box<Location>),
}

/// Geocoding search result.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CityResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub country: Option<String>,
    #[serde(default)]
    pub admin1: Option<String>,
    #[serde(default)]
    pub elevation: Option<f64>,
}

impl ReturnedData {
    pub fn as_daily(&self) -> Option<&WeatherDaily> {
        if let Self::Daily(d) = self {
            Some(d)
        } else {
            None
        }
    }

    pub fn as_current(&self) -> Option<&WeatherResponse> {
        if let Self::Current(c) = self {
            Some(c)
        } else {
            None
        }
    }

    pub fn as_location(&self) -> Option<&Location> {
        if let Self::Location(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_hourly(&self) -> Option<&WeatherHourly> {
        if let Self::Hourly(h) = self {
            Some(h)
        } else {
            None
        }
    }
}

/// Hourly forecast response.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherHourly {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HourlyUnits {
    pub time: String,
    pub temperature_2m: String,
    pub precipitation_probability: String,
    pub weather_code: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation_probability: Vec<u32>,
    pub weather_code: Vec<u32>,
}

/// Map a WMO weather code to a human-readable description.
pub fn weather_code_to_text(code: u32) -> &'static str {
    match code {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 | 48 => "Fog",
        51 => "Light drizzle",
        53 => "Moderate drizzle",
        55 => "Dense drizzle",
        56 => "Light freezing drizzle",
        57 => "Dense freezing drizzle",
        61 => "Slight rain",
        63 => "Moderate rain",
        65 => "Heavy rain",
        66 => "Light freezing rain",
        67 => "Heavy freezing rain",
        71 => "Slight snowfall",
        73 => "Moderate snowfall",
        75 => "Heavy snowfall",
        77 => "Snow grains",
        80 => "Slight rain showers",
        81 => "Moderate rain showers",
        82 => "Violent rain showers",
        85 => "Slight snow showers",
        86 => "Heavy snow showers",
        95 => "Thunderstorm",
        96 => "Thunderstorm with slight hail",
        99 => "Thunderstorm with heavy hail",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_weather_response() {
        let json = r#"{
            "latitude": 48.86,
            "longitude": 2.35,
            "generationtime_ms": 0.5,
            "timezone": "GMT",
            "timezone_abbreviation": "GMT",
            "elevation": 35.0,
            "current_units": {
                "temperature_2m": "°C",
                "apparent_temperature": "°C",
                "relative_humidity_2m": "%",
                "surface_pressure": "hPa",
                "wind_speed_10m": "km/h",
                "uv_index": "",
                "weather_code": "",
                "rain": "mm",
                "snowfall": "cm",
                "precipitation": "mm"
            },
            "current": {
                "time": "2025-07-26T12:00",
                "temperature_2m": 22.5,
                "apparent_temperature": 21.0,
                "relative_humidity_2m": 65.0,
                "surface_pressure": 1013.0,
                "wind_speed_10m": 12.3,
                "uv_index": 5.0,
                "weather_code": 1,
                "rain": 0.0,
                "snowfall": 0.0,
                "precipitation": 0.0
            }
        }"#;
        let resp: WeatherResponse = serde_json::from_str(json).unwrap();
        assert!((resp.latitude - 48.86).abs() < 0.01);
        assert!((resp.current.temperature_2m - 22.5).abs() < 0.01);
        assert!((resp.current.apparent_temperature - 21.0).abs() < 0.01);
        assert_eq!(resp.current.weather_code, 1);
        assert_eq!(resp.timezone, "GMT");
    }

    #[test]
    fn deserialize_weather_daily() {
        let json = r#"{
            "latitude": 48.86,
            "longitude": 2.35,
            "generationtime_ms": 1.2,
            "utc_offset_seconds": 0,
            "timezone": "GMT",
            "timezone_abbreviation": "GMT",
            "elevation": 35.0,
            "daily_units": {
                "time": "iso8601",
                "uv_index_max": "",
                "snowfall_sum": "cm",
                "showers_sum": "mm",
                "rain_sum": "mm",
                "shortwave_radiation_sum": "MJ/m²",
                "temperature_2m_mean": "°C",
                "wind_speed_10m_max": "km/h"
            },
            "daily": {
                "time": ["2025-07-26", "2025-07-27"],
                "uv_index_max": [8.5, 7.2],
                "snowfall_sum": [0.0, 0.0],
                "showers_sum": [0.0, 1.2],
                "rain_sum": [0.0, 2.5],
                "shortwave_radiation_sum": [25.0, 22.0],
                "temperature_2m_mean": [22.0, 19.5],
                "wind_speed_10m_max": [15.0, 20.0]
            }
        }"#;
        let resp: WeatherDaily = serde_json::from_str(json).unwrap();
        assert_eq!(resp.daily.time.len(), 2);
        assert_eq!(resp.daily.temperature_2m_mean[0], 22.0);
        assert_eq!(resp.daily.wind_speed_10m_max[1], 20.0);
    }

    #[test]
    fn deserialize_location() {
        let json = r#"{"lat": 51.51, "lon": -0.13}"#;
        let loc: Location = serde_json::from_str(json).unwrap();
        assert!((loc.lat - 51.51).abs() < 0.01);
        assert!((loc.lon - (-0.13)).abs() < 0.01);
    }

    #[test]
    fn returned_data_roundtrip() {
        let json = r#"{"lat": 40.71, "lon": -74.01}"#;
        let loc: Location = serde_json::from_str(json).unwrap();
        let data = ReturnedData::Location(Box::new(loc.clone()));
        assert!(data.as_location().is_some());
        assert!(data.as_daily().is_none());
        assert!(data.as_current().is_none());
        assert!(data.as_hourly().is_none());

        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: ReturnedData = serde_json::from_str(&serialized).unwrap();
        assert!((deserialized.as_location().unwrap().lat - 40.71).abs() < 0.01);
    }

    #[test]
    fn deserialize_hourly() {
        let json = r#"{
            "latitude": 48.86,
            "longitude": 2.35,
            "generationtime_ms": 0.5,
            "timezone": "GMT",
            "timezone_abbreviation": "GMT",
            "elevation": 35.0,
            "hourly_units": {
                "time": "iso8601",
                "temperature_2m": "°C",
                "precipitation_probability": "%",
                "weather_code": ""
            },
            "hourly": {
                "time": ["2025-07-26T00:00", "2025-07-26T01:00"],
                "temperature_2m": [20.0, 19.5],
                "precipitation_probability": [10, 25],
                "weather_code": [0, 2]
            }
        }"#;
        let resp: WeatherHourly = serde_json::from_str(json).unwrap();
        assert_eq!(resp.hourly.time.len(), 2);
        assert!((resp.hourly.temperature_2m[0] - 20.0).abs() < 0.01);
        assert_eq!(resp.hourly.precipitation_probability[1], 25);
        assert_eq!(resp.hourly.weather_code[0], 0);
    }

    #[test]
    fn deserialize_city_result() {
        let json = r#"{"name": "Paris", "latitude": 48.8566, "longitude": 2.3522, "country": "France", "admin1": "Ile-de-France"}"#;
        let city: CityResult = serde_json::from_str(json).unwrap();
        assert_eq!(city.name, "Paris");
        assert_eq!(city.country, Some("France".into()));
        assert_eq!(city.admin1, Some("Ile-de-France".into()));
    }

    #[test]
    fn weather_code_mapping() {
        assert_eq!(weather_code_to_text(0), "Clear sky");
        assert_eq!(weather_code_to_text(3), "Overcast");
        assert_eq!(weather_code_to_text(45), "Fog");
        assert_eq!(weather_code_to_text(61), "Slight rain");
        assert_eq!(weather_code_to_text(95), "Thunderstorm");
        assert_eq!(weather_code_to_text(999), "Unknown");
    }
}
