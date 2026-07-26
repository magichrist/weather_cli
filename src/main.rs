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
    #[arg(short, requires = "b", conflicts_with = "interactive")]
    a: Option<f64>,

    /// Longitude (requires -a)
    #[arg(short, requires = "a", conflicts_with = "interactive")]
    b: Option<f64>,

    /// Use IP-based geolocation
    #[arg(long, conflicts_with = "interactive")]
    ml: bool,

    /// Clear the local cache
    #[arg(short = 'c', long = "clear_cache")]
    clear_cache: bool,

    /// Interactive mode
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,

    /// Show 7-day forecast instead of current weather
    #[arg(short = 'f', long = "forecast")]
    forecast: bool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();
    debug!(?args, "parsed arguments");

    let result = if let (Some(lat), Some(lon)) = (args.a, args.b) {
        app::handle_direct(lat, lon, args.forecast).await
    } else if args.interactive {
        app::handle_interactive(args.forecast).await
    } else if args.ml {
        app::handle_my_location(args.forecast).await
    } else if args.clear_cache {
        cache::Cache::clear()
    } else {
        eprintln!("No action specified. Use --help for usage.");
        std::process::exit(1);
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
