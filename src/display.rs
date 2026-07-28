use crate::models::{
    CityResult, WeatherDaily, WeatherHourly, WeatherResponse, weather_code_to_text,
};
use colored::Colorize;
use std::fmt::Write as _;
use std::io::Write;
use textplots::{Chart, Plot, Shape};

/// Display a temperature chart for the forecast period.
pub fn depict_forecast(data: &WeatherDaily) {
    if data.daily.time.is_empty() {
        return;
    }

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
        "  {} {}",
        "Condition:".bright_blue(),
        weather_code_to_text(data.current.weather_code)
    )
    .ok();
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
        "Feels like:".bright_red(),
        data.current.apparent_temperature,
        data.current_units.apparent_temperature
    )
    .ok();
    writeln!(
        out,
        "  {} {} {}",
        "Humidity:".bright_blue(),
        data.current.relative_humidity_2m,
        data.current_units.relative_humidity_2m
    )
    .ok();
    writeln!(
        out,
        "  {} {} {}",
        "Pressure:".bright_blue(),
        data.current.surface_pressure,
        data.current_units.surface_pressure
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
        "  {} {:.1} {}",
        "UV Index:".purple(),
        data.current.uv_index,
        data.current_units.uv_index
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

/// Print city search results for user disambiguation.
pub fn print_city_results(results: &[CityResult]) {
    let mut out = String::new();
    writeln!(out, "{}", "City Search Results:".green()).ok();
    for (i, r) in results.iter().enumerate() {
        let location = match (&r.country, &r.admin1) {
            (Some(c), Some(a)) => format!("{}, {}", a, c),
            (Some(c), None) => c.clone(),
            _ => String::new(),
        };
        writeln!(
            out,
            "  {}. {} {} ({:.4}, {:.4})",
            (i + 1).to_string().bright_blue(),
            r.name.yellow(),
            location.dimmed(),
            r.latitude,
            r.longitude,
        )
        .ok();
    }
    print!("{out}");
    let _ = std::io::stdout().flush();
}

/// Print hourly forecast table.
pub fn pretty_print_hourly(data: &WeatherHourly) {
    let mut out = String::new();

    writeln!(out, "{}", "Hourly Forecast:".green()).ok();
    writeln!(
        out,
        "  {} {}, {}",
        "Location:".bright_blue(),
        data.latitude,
        data.longitude
    )
    .ok();
    writeln!(out).ok();

    writeln!(
        out,
        "{:<20} {:<10} {:<12} {:<10}",
        "Time".yellow(),
        "Temp °C".bright_red(),
        "Precip %".bright_blue(),
        "Condition".bright_green(),
    )
    .ok();

    for i in 0..data.hourly.time.len() {
        let time_str = &data.hourly.time[i];
        let short_time = time_str.get(11..16).unwrap_or(time_str);
        let condition = weather_code_to_text(data.hourly.weather_code[i]);
        writeln!(
            out,
            "{:<20} {:<10.1} {:<12} {:<10}",
            short_time.bright_white(),
            data.hourly.temperature_2m[i],
            data.hourly.precipitation_probability[i],
            condition,
        )
        .ok();
    }

    print!("{out}");
    let _ = std::io::stdout().flush();
}
