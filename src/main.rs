mod cache;
mod connectors;
mod depictors;
mod response_layouts;

use crate::cache::{insert_save, is_valid, load_cache};
use crate::connectors::{fetch, transform_url};
use crate::depictors::{depict_forecast, pretty_print_forecast, pretty_print_weather};
use clap::Parser;
use colored::Colorize;
use prompts::Prompt;
use prompts::text::TextPrompt;
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Latitude (must be used with -b)
    #[arg(short, requires = "b", conflicts_with = "interactive")]
    a: Option<f32>,

    /// Longitude (must be used with -a)
    #[arg(short, requires = "a", conflicts_with = "interactive")]
    b: Option<f32>,

    /// Use IP-based geolocation
    #[arg(long, conflicts_with = "interactive")]
    ml: bool,

    /// Clear cache
    #[arg(short = 'c', long = "clear_cache")]
    clear_cache: bool,

    /// Interactive mode
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,

    /// Forecast mode
    #[arg(short = 'f', long = "forecast")]
    forecast: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    debug!(
        "Lat: {} Lon: {} interactive: {} forecast: {}",
        args.a.unwrap_or(0.0),
        args.b.unwrap_or(0.0),
        args.interactive,
        args.forecast
    );

    let result = if let (Some(a), Some(b)) = (args.a, args.b) {
        calc_and_fetch(a, b, args.forecast).await
    } else if args.interactive {
        interactive_loop(args.forecast).await
    } else if args.ml {
        fetch_my_location(args.forecast).await
    } else if args.clear_cache {
        cache::clear_cache();
        Ok(())
    } else {
        eprintln!("No valid arguments provided. Use --help for usage.");
        std::process::exit(1);
    };

    if let Err(e) = result {
        eprintln!("{e}");
    }
}

/// Fetch weather for a given lat/lon, checking cache first.
async fn calc_and_fetch(
    lat: f32,
    lon: f32,
    forecast: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    debug!("lat_lon validation: {lat} {lon}");

    if !(-90.0..=90.0).contains(&lat) {
        println!("Latitude must be between -90 and 90, got {lat}");
        return Ok(());
    }
    if !(-180.0..=180.0).contains(&lon) {
        println!("Longitude must be between -180 and 180, got {lon}");
        return Ok(());
    }

    let lat_lon = [lat, lon];
    let lat_lon_string = format!("{}_{}", lat_lon[0], lat_lon[1]);
    let mut cache_file = load_cache();

    if forecast {
        let api_hook = "https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&daily=uv_index_max,snowfall_sum,showers_sum,rain_sum,shortwave_radiation_sum,temperature_2m_mean,wind_speed_10m_max&timezone=GMT";
        let url = transform_url(api_hook, &lat_lon);
        let key = format!("{lat_lon_string}forecast");

        if let Some(entry) = cache_file.get(&key)
            && is_valid(entry)
            && let Some(daily) = entry.data.as_daily()
        {
            pretty_print_forecast(daily);
            depict_forecast(daily);
            return Ok(());
        }

        let fetched_data = fetch(url, "Forecasting").await?;
        if let Some(daily) = fetched_data.as_daily() {
            pretty_print_forecast(daily);
            depict_forecast(daily);
            insert_save(key, fetched_data.clone(), &mut cache_file);
        } else {
            eprintln!("API returned unexpected data for forecast request");
        }
    } else {
        let api_hook = "https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&current=temperature_2m,wind_speed_10m,rain,snowfall,precipitation";
        let url = transform_url(api_hook, &lat_lon);
        let key = format!("{lat_lon_string}current");

        if let Some(entry) = cache_file.get(&key)
            && is_valid(entry)
            && let Some(current) = entry.data.as_current()
        {
            pretty_print_weather(current);
            return Ok(());
        }

        let fetched_data = fetch(url, "Getting Current State").await?;
        if let Some(current) = fetched_data.as_current() {
            insert_save(key, fetched_data.clone(), &mut cache_file);
            pretty_print_weather(current);
        } else {
            eprintln!("API returned unexpected data for current weather request");
        }
    }

    Ok(())
}

/// Fetch location from IP geolocation API, then fetch weather.
async fn fetch_my_location(forecast: bool) -> Result<(), Box<dyn std::error::Error>> {
    let ip_url = "http://ip-api.com/json?fields=lat,lon";
    let mut cache_file = load_cache();

    if let Some(cache_hit) = cache_file.get("ml")
        && is_valid(cache_hit)
        && let Some(loc) = cache_hit.data.as_location()
    {
        calc_and_fetch(loc.lat as f32, loc.lon as f32, forecast).await?;
        return Ok(());
    }

    let mylocation = fetch(ip_url.to_string(), "Getting Location").await?;
    if let Some(loc) = mylocation.as_location() {
        insert_save("ml".to_string(), mylocation.clone(), &mut cache_file);
        calc_and_fetch(loc.lat as f32, loc.lon as f32, forecast).await?;
    } else {
        eprintln!("Could not determine location from IP");
    }

    Ok(())
}

/// Interactive REPL for entering coordinates.
async fn interactive_loop(forecast: bool) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut prompt = TextPrompt::new("$ ");
        match prompt.run().await {
            Ok(Some(data)) => {
                debug!("Raw Data: {data:?}");
                if data == "q" {
                    break;
                } else if data == "help" {
                    println!("{}", "Enter LAT then LON".green());
                    println!("Example: ");
                    println!("{}", "1.2 2.3".bright_red());
                } else if data.trim().is_empty() {
                    println!("{} {data}", "Wrong Input:".green());
                } else {
                    let parts: Vec<f32> = data
                        .split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect();
                    if parts.len() == 2 {
                        calc_and_fetch(parts[0], parts[1], forecast).await?;
                    } else {
                        println!("{}", "Expected two numbers: LAT LON".bright_red());
                    }
                }
            }
            other => {
                println!("exiting {other:?}");
                break;
            }
        }
    }
    Ok(())
}
