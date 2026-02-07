use crate::response_layouts::{WeatherDaily, WeatherResponse};
use colored::Colorize;
use textplots::{Chart, Plot, Shape};

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
    println!("{}", "Time-Temp Chart".red());
    Chart::new(width, height, 0.0, 6.0)
        .lineplot(&Shape::Steps(&temp_points))
        .display();
}

pub fn pretty_print_forecast(weather: &WeatherDaily) {
    println!(
        "{:<12} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10} {:<10}",
        "Date".yellow(),
        "UV Max".purple(),
        "Snow cm".white(),
        "Showers mm".cyan(),
        "Rain mm".bright_blue(),
        "Radiation".bright_yellow(),
        "Temp °C".bright_red(),
        "Wind km/h".bright_green()
    );

    for i in 0..weather.daily.time.len() {
        println!(
            "{:<12} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<10.2} {:<10.2}",
            weather.daily.time[i].yellow(),
            weather.daily.uv_index_max[i],
            weather.daily.snowfall_sum[i],
            weather.daily.showers_sum[i],
            weather.daily.rain_sum[i],
            weather.daily.shortwave_radiation_sum[i],
            weather.daily.temperature_2m_mean[i],
            weather.daily.wind_speed_10m_max[i],
        );
    }
}

// Current

pub fn pretty_print_weather(data: &WeatherResponse) {
    println!("{}", "🌍 Weather Data:".green());
    println!("Location: {}, {}", data.latitude, data.longitude);
    println!(
        "Timezone: {} ({})",
        data.timezone, data.timezone_abbreviation
    );
    println!("Elevation: {} m", data.elevation);
    println!("Generated in: {:.2} ms", data.generationtime_ms);

    println!("{}", "\n🌡️ Current Conditions:".red());
    println!("Time: {}", data.current.time);
    println!(
        "Temperature: {} {}",
        data.current.temperature_2m, data.current_units.temperature_2m
    );
    println!(
        "Wind Speed: {} {}",
        data.current.wind_speed_10m, data.current_units.wind_speed_10m
    );
    println!("Rain: {} {}", data.current.rain, data.current_units.rain);
    println!(
        "Snowfall: {} {}",
        data.current.snowfall, data.current_units.snowfall
    );
    println!(
        "Precipitation: {} {}",
        data.current.precipitation, data.current_units.precipitation
    );
}
