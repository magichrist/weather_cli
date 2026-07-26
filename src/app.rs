use crate::api;
use crate::cache::Cache;
use crate::display;
use crate::error::{AppError, AppResult};
use colored::Colorize;
use prompts::Prompt;
use prompts::text::TextPrompt;
use tracing::debug;

/// Fetch and display weather for the given coordinates.
pub async fn handle_direct(lat: f64, lon: f64, forecast: bool) -> AppResult<()> {
    validate_coords(lat, lon)?;
    let mut cache = Cache::load();
    handle_direct_with_cache(lat, lon, forecast, &mut cache).await
}

async fn handle_direct_with_cache(
    lat: f64,
    lon: f64,
    forecast: bool,
    cache: &mut Cache,
) -> AppResult<()> {
    let cache_key = format!("{lat}_{lon}_{}", mode_suffix(forecast));

    if let Some(data) = cache.get_valid(&cache_key) {
        if display_cached(data, forecast) {
            return Ok(());
        }
    }

    if forecast {
        let data = api::fetch_forecast(lat, lon).await?;
        display::pretty_print_forecast(&data);
        display::depict_forecast(&data);
        cache.insert(
            &cache_key,
            crate::models::ReturnedData::Daily(Box::new(data)),
        );
    } else {
        let data = api::fetch_current(lat, lon).await?;
        display::pretty_print_weather(&data);
        cache.insert(
            &cache_key,
            crate::models::ReturnedData::Current(Box::new(data)),
        );
    }

    Ok(())
}

/// Determine location from IP, then fetch weather.
pub async fn handle_my_location(forecast: bool) -> AppResult<()> {
    let mut cache = Cache::load();
    let loc = if let Some(data) = cache.get_valid("ml") {
        data.as_location()
            .cloned()
            .ok_or(AppError::UnexpectedResponse)?
    } else {
        let loc = api::fetch_location().await?;
        cache.insert(
            "ml",
            crate::models::ReturnedData::Location(Box::new(loc.clone())),
        );
        loc
    };

    handle_direct_with_cache(loc.lat, loc.lon, forecast, &mut cache).await
}

/// Interactive REPL for entering coordinates.
pub async fn handle_interactive(forecast: bool) -> AppResult<()> {
    println!(
        "{}",
        "Interactive mode — type LAT LON, 'help', or 'q' to quit".green()
    );

    loop {
        let mut prompt = TextPrompt::new("$ ");
        match prompt.run().await {
            Ok(Some(input)) => {
                debug!("raw input: {input:?}");
                match input.trim() {
                    "q" | "quit" | "exit" => break,
                    "help" => print_help(),
                    "" => {
                        println!(
                            "{}",
                            "Empty input — type LAT LON (e.g. 48.85 2.35)".yellow()
                        );
                    }
                    _ => match parse_coords(&input) {
                        Ok((lat, lon)) => {
                            if let Err(e) = handle_direct(lat, lon, forecast).await {
                                eprintln!("  {e}");
                            }
                        }
                        Err(e) => eprintln!("  {e}"),
                    },
                }
            }
            _ => break,
        }
    }

    Ok(())
}

// -- private helpers ------------------------------------------------------

fn validate_coords(lat: f64, lon: f64) -> AppResult<()> {
    if !(-90.0..=90.0).contains(&lat) {
        return Err(AppError::InvalidLatitude { lat });
    }
    if !(-180.0..=180.0).contains(&lon) {
        return Err(AppError::InvalidLongitude { lon });
    }
    Ok(())
}

fn parse_coords(input: &str) -> AppResult<(f64, f64)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(AppError::Cache("expected two numbers: LAT LON".to_string()));
    }
    let lat: f64 = parts[0]
        .parse()
        .map_err(|_| AppError::Cache(format!("invalid latitude: {}", parts[0])))?;
    let lon: f64 = parts[1]
        .parse()
        .map_err(|_| AppError::Cache(format!("invalid longitude: {}", parts[1])))?;
    validate_coords(lat, lon)?;
    Ok((lat, lon))
}

/// Try to display a cached entry. Returns `true` if successful.
fn display_cached(data: &crate::models::ReturnedData, forecast: bool) -> bool {
    if forecast {
        if let Some(daily) = data.as_daily() {
            display::pretty_print_forecast(daily);
            display::depict_forecast(daily);
            return true;
        }
    } else if let Some(current) = data.as_current() {
        display::pretty_print_weather(current);
        return true;
    }
    false
}

fn mode_suffix(forecast: bool) -> &'static str {
    if forecast { "forecast" } else { "current" }
}

fn print_help() {
    println!("{}", "Commands:".bright_blue());
    println!("  {} — enter coordinates as LAT LON", "<LAT> <LON>".green());
    println!("  {} — show this help", "help".green());
    println!("  {} — exit", "q".green());
    println!();
    println!("Examples:");
    println!("  {}  — Paris", "48.85 2.35".bright_red());
    println!("  {} — New York", "40.71 -74.01".bright_red());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_coords_ok() {
        assert!(validate_coords(48.85, 2.35).is_ok());
        assert!(validate_coords(-90.0, -180.0).is_ok());
        assert!(validate_coords(90.0, 180.0).is_ok());
        assert!(validate_coords(0.0, 0.0).is_ok());
    }

    #[test]
    fn validate_coords_latitude_out_of_range() {
        let err = validate_coords(91.0, 0.0).unwrap_err();
        assert!(matches!(err, AppError::InvalidLatitude { lat } if lat == 91.0));
    }

    #[test]
    fn validate_coords_longitude_out_of_range() {
        let err = validate_coords(0.0, -181.0).unwrap_err();
        assert!(matches!(err, AppError::InvalidLongitude { lon } if lon == -181.0));
    }

    #[test]
    fn parse_coords_valid() {
        let (lat, lon) = parse_coords("48.85 2.35").unwrap();
        assert!((lat - 48.85).abs() < 0.001);
        assert!((lon - 2.35).abs() < 0.001);
    }

    #[test]
    fn parse_coords_negative() {
        let (lat, lon) = parse_coords("-33.87 151.21").unwrap();
        assert!((lat - (-33.87)).abs() < 0.01);
        assert!((lon - 151.21).abs() < 0.01);
    }

    #[test]
    fn parse_coords_wrong_count() {
        assert!(parse_coords("48.85").is_err());
        assert!(parse_coords("48.85 2.35 extra").is_err());
        assert!(parse_coords("").is_err());
    }

    #[test]
    fn parse_coords_non_numeric() {
        assert!(parse_coords("abc def").is_err());
    }

    #[test]
    fn mode_suffix_forecast() {
        assert_eq!(mode_suffix(true), "forecast");
        assert_eq!(mode_suffix(false), "current");
    }

    #[test]
    fn display_cached_current_miss_on_forecast() {
        let data = crate::models::ReturnedData::Current(Box::new(crate::models::WeatherResponse {
            latitude: 0.0,
            longitude: 0.0,
            generationtime_ms: 0.0,
            timezone: "GMT".into(),
            timezone_abbreviation: "GMT".into(),
            elevation: 0.0,
            current_units: crate::models::CurrentUnits {
                temperature_2m: "°C".into(),
                wind_speed_10m: "km/h".into(),
                rain: "mm".into(),
                snowfall: "cm".into(),
                precipitation: "mm".into(),
            },
            current: crate::models::Current {
                time: "2025-01-01T00:00".into(),
                temperature_2m: 10.0,
                wind_speed_10m: 5.0,
                rain: 0.0,
                snowfall: 0.0,
                precipitation: 0.0,
            },
        }));
        // Asking for forecast but data is Current — should return false
        assert!(!display_cached(&data, true));
    }
}
