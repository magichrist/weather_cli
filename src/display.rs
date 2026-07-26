use crate::models::{WeatherDaily, WeatherResponse};
use colored::Colorize;
use std::fmt::Write as _;
use std::io::Write;
use textplots::{Chart, Plot, Shape};

/// Display a temperature chart for the forecast period.
pub fn depict_forecast(data: &WeatherDaily) {
    let temp_points: Vec<(f32, f32)> = data
        .daily
        .temperature_2m_mean
        .iter()
        .enumerate()
        .map(|(i, &v)| (i as f32, v as f32))
        .collect();

    let width = 80;
    let height = 20;
    let x_max = (data.daily.time.len() as f32) - 1.0;

    println!("{}", "Temperature Forecast".red());
    Chart::new(width, height, 0.0, x_max)
        .lineplot(&Shape::Steps(&temp_points))
        .display();
}

/// Print the 7-day forecast table.
pub fn pretty_print_forecast(weather: &WeatherDaily) {
    let mut out = String::new();

    writeln!(
        out,
        "{:<12} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
        "Date".yellow(),
        "UV Max".purple(),
        "Snow cm".white(),
        "Showers".cyan(),
        "Rain mm".bright_blue(),
        "Radiation".bright_yellow(),
        "Temp °C".bright_red(),
        "Wind km/h".bright_green()
    )
    .ok();

    for (i, date) in weather.daily.time.iter().enumerate() {
        let row = format!(
            "{:<12} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<10.2}",
            date.yellow(),
            weather.daily.uv_index_max[i],
            weather.daily.snowfall_sum[i],
            weather.daily.showers_sum[i],
            weather.daily.rain_sum[i],
            weather.daily.shortwave_radiation_sum[i],
            weather.daily.temperature_2m_mean[i],
            weather.daily.wind_speed_10m_max[i]
        );
        if i % 2 == 0 {
            writeln!(out, "{}", row.green()).ok();
        } else {
            writeln!(out, "{row}").ok();
        }
    }

    print!("{out}");
    let _ = std::io::stdout().flush();
}

/// Print current weather conditions.
pub fn pretty_print_weather(data: &WeatherResponse) {
    let mut out = String::new();

    writeln!(out, "{}", "Current Weather Data:".green()).ok();
    writeln!(
        out,
        "  {} {}, {}",
        "Location:".bright_blue(),
        data.latitude,
        data.longitude
    )
    .ok();
    writeln!(
        out,
        "  {} {} ({})",
        "Timezone:".bright_blue(),
        data.timezone,
        data.timezone_abbreviation
    )
    .ok();
    writeln!(out, "  {} {} m", "Elevation:".bright_blue(), data.elevation).ok();
    writeln!(
        out,
        "  {} {:.2} ms",
        "Generated in:".bright_blue(),
        data.generationtime_ms
    )
    .ok();
    writeln!(out).ok();
    writeln!(out, "{}", "Current Conditions:".bright_blue()).ok();
    writeln!(out, "  {} {}", "Time:".bright_blue(), data.current.time).ok();
    writeln!(
        out,
        "  {} {} {}",
        "Temperature:".bright_red(),
        data.current.temperature_2m,
        data.current_units.temperature_2m
    )
    .ok();
    writeln!(
        out,
        "  {} {} {}",
        "Wind Speed:".bright_green(),
        data.current.wind_speed_10m,
        data.current_units.wind_speed_10m
    )
    .ok();
    writeln!(
        out,
        "  {} {} {}",
        "Rain:".bright_blue(),
        data.current.rain,
        data.current_units.rain
    )
    .ok();
    writeln!(
        out,
        "  {} {} {}",
        "Snowfall:".white(),
        data.current.snowfall,
        data.current_units.snowfall
    )
    .ok();
    writeln!(
        out,
        "  {} {} {}",
        "Precipitation:".cyan(),
        data.current.precipitation,
        data.current_units.precipitation
    )
    .ok();

    print!("{out}");
    let _ = std::io::stdout().flush();
}
