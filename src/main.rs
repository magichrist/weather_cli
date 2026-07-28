//! A weather CLI tool for current conditions and 7-day forecasts.
//!
//! Uses the Open-Meteo API for weather data and supports IP-based geolocation.

#![warn(missing_docs)]

mod api;
mod app;
mod cache;
mod display;
mod error;
mod models;

use clap::Parser;
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(
    name = "weather_cli",
    author,
    version,
    about = "A weather CLI tool for current conditions and 7-day forecasts"
)]
struct Args {
    /// Latitude (requires -b)
    #[arg(short, requires = "b", conflicts_with_all = ["interactive", "search"])]
    a: Option<f64>,

    /// Longitude (requires -a)
    #[arg(short, requires = "a", conflicts_with_all = ["interactive", "search"])]
    b: Option<f64>,

    /// Search by city name
    #[arg(short = 's', long = "search", conflicts_with_all = ["a", "b", "interactive"])]
    search: Option<String>,

    /// Use IP-based geolocation
    #[arg(long, conflicts_with_all = ["interactive", "search"])]
    ml: bool,

    /// Clear the local cache
    #[arg(short = 'c', long = "clear_cache")]
    clear_cache: bool,

    /// Interactive mode
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,

    /// Show 7-day forecast instead of current weather
    #[arg(short = 'f', long = "forecast", conflicts_with = "hourly")]
    forecast: bool,

    /// Show hourly forecast (optional: number of days, default 3)
    #[arg(long = "hourly", num_args = 0..=1, default_missing_value = "3", conflicts_with = "forecast")]
    hourly: Option<u32>,

    /// Output as JSON
    #[arg(long)]
    json: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    debug!(?args, "parsed arguments");

    let result = run(args).await;

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

async fn run(args: Args) -> error::AppResult<()> {
    if let Some(city) = &args.search {
        app::handle_city_search(city, args.forecast, args.hourly, args.json).await?;
    } else if let Some(days) = args.hourly {
        let (lat, lon) = resolve_coords(args.a, args.b, args.ml).await?;
        app::handle_hourly(lat, lon, days, args.json).await?;
    } else if args.json {
        let (lat, lon) = resolve_coords(args.a, args.b, args.ml).await?;
        app::handle_json(lat, lon, args.forecast).await?;
    } else if let (Some(lat), Some(lon)) = (args.a, args.b) {
        app::handle_direct(lat, lon, args.forecast).await?;
    } else if args.interactive {
        app::handle_interactive(args.forecast).await?;
    } else if args.ml {
        app::handle_my_location(args.forecast).await?;
    } else if args.clear_cache {
        cache::Cache::clear()?;
    } else {
        eprintln!("No action specified. Use --help for usage.");
        std::process::exit(1);
    }
    Ok(())
}

async fn resolve_coords(a: Option<f64>, b: Option<f64>, ml: bool) -> error::AppResult<(f64, f64)> {
    if let (Some(lat), Some(lon)) = (a, b) {
        return Ok((lat, lon));
    }
    if ml {
        let loc = app::resolve_location().await?;
        return Ok((loc.lat, loc.lon));
    }
    Err(error::AppError::Cache(
        "no coordinates provided (use -a/-b, --ml, or -s)".to_string(),
    ))
}
