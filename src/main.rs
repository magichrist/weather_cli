mod cache;
mod connectors;
mod depictors;
mod response_layouts;

use crate::cache::{insert_save, is_valid, load_cache};
#[allow(unused)]
use crate::connectors::{ReturnedData, fetch, transform_url};
use crate::depictors::{depict_forecast, pretty_print_forecast, pretty_print_weather};
use clap::Parser;
use colored::Colorize;
use prompts::Prompt;
use prompts::text::TextPrompt;
use std::process::exit;
use tracing::debug;
use tracing_subscriber::EnvFilter;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Lat (must be used with -b)
    #[arg(short, requires = "b", conflicts_with = "interactive")]
    a: Option<f32>,

    /// Lon (must be used with -a)
    #[arg(short, requires = "a", conflicts_with = "interactive")]
    b: Option<f32>,

    /// mylocation
    #[arg(long, conflicts_with = "interactive")]
    ml: bool,

    /// Clear Cache
    #[arg(short = 'c', long = "clear_cache")]
    clear_cache: bool,

    /// Interactive mode
    #[arg(short = 'i', long = "interactive")]
    interactive: bool,

    /// Forecast mode
    #[arg(short = 'f', long = "forecast")]
    forecast: bool,
}
#[allow(dead_code)]
#[derive(Debug)]
enum BadInput {
    MoreThanTwo,
    LatNotInRange,
    LonNotInRange,
}
#[allow(clippy::collapsible_if)]
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
    if !args.interactive && args.a.is_some() && args.b.is_some() {
        let data = format!("{} {}", args.a.unwrap(), args.b.unwrap());
        calc_and_fetch(data).await;
    } else if args.interactive {
        loop {
            let mut prompt = TextPrompt::new("$ ");
            match prompt.run().await {
                Ok(Some(data)) => {
                    debug!("Raw Data: {:?}", data);
                    if data == "q" {
                        break;
                    } else if data == "help" {
                        println!("{}", "LAT then LON".green());
                        println!("Like: ");
                        println!("{}", "1.2 2.3".bright_red());
                    } else if data.trim().is_empty() {
                        println!("{} {}", "Wrong Input: ".green(), data);
                    } else {
                        calc_and_fetch(data).await;
                    }
                }
                other => {
                    println!("exiting {:?}", other);
                    break;
                }
            }
        }
    } else if args.ml && !args.interactive && args.b.is_none() && args.a.is_none() {
        let mut cache_file = load_cache();
        if let Some(cache_hit) = cache_file.get("ml") {
            if is_valid(cache_hit) {
                let loc = cache_hit.data.mlocation().unwrap();
                calc_and_fetch(format!("{} {}", loc.lat, loc.lon)).await;
                return;
            }
        }
        let mylocation = fetch("http://ip-api.com/json?fields=lat,lon".to_string())
            .await
            .unwrap();
        insert_save("ml".to_string(), mylocation.clone(), &mut cache_file);
        calc_and_fetch(format!(
            "{} {}",
            mylocation.mlocation().unwrap().lat,
            mylocation.mlocation().unwrap().lon
        ))
        .await;
    } else if args.clear_cache {
        cache::clear_cache();
    } else {
        exit(1);
    }
}

#[allow(clippy::collapsible_if, clippy::collapsible_else_if)]
async fn calc_and_fetch(data: String) {
    debug!("lat_lon transformation");
    let lat_lon: Vec<f32> = data
        .split_whitespace()
        .filter_map(|x| x.parse::<f32>().ok())
        .collect();
    let mut cache_file = cache::load_cache();
    if lat_lon.len() < 2 {
        return;
    }
    let lat_lon_string = format!("{}_{}", lat_lon[0], lat_lon[1]);
    debug!("Transformed: {:?}", lat_lon);
    if lat_lon.len() != 2 {
        println!("{} {}", "Wrong Input: ".green(), data);
    } else if lat_lon[0] < -90.0 || lat_lon[0] > 90.0 {
        println!("{:?}", BadInput::LatNotInRange);
    } else if lat_lon[1] < -180.0 || lat_lon[1] > 180.0 {
        println!("{:?}", BadInput::LonNotInRange);
    } else {
        if Args::parse().forecast {
            let mut api_hook:String="https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&daily=uv_index_max,snowfall_sum,showers_sum,rain_sum,shortwave_radiation_sum,temperature_2m_mean,wind_speed_10m_max&timezone=GMT".into();
            api_hook = transform_url(&api_hook, &lat_lon);
            let key = format!("{}forecast", lat_lon_string);
            if let Some(entry) = cache_file.get(&key) {
                if is_valid(entry) {
                    let cache_hit = entry.data.daily().unwrap();
                    pretty_print_forecast(cache_hit);
                    depict_forecast(cache_hit);
                    return;
                }
            }
            let fetched_data = fetch(api_hook).await.unwrap();
            pretty_print_forecast(fetched_data.daily().unwrap());
            cache::insert_save(key, fetched_data.clone(), &mut cache_file);
            depict_forecast(fetched_data.daily().unwrap());
        } else {
            let mut api_hook:String = "https://api.open-meteo.com/v1/forecast?latitude=LAT&longitude=LON&current=temperature_2m,wind_speed_10m,rain,snowfall,precipitation".into();
            api_hook = transform_url(&api_hook, &lat_lon);
            let key = format!("{}current", lat_lon_string);
            if let Some(entry) = cache_file.get(&key) {
                if is_valid(entry) {
                    let cache_hit = entry.data.current().unwrap();
                    pretty_print_weather(cache_hit);
                    return;
                }
            }
            let fetched_data = fetch(api_hook).await.unwrap();
            insert_save(key, fetched_data.clone(), &mut cache_file);
            pretty_print_weather(fetched_data.current().unwrap());
        }
    }
}
