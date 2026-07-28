# weather_cli

A weather CLI tool for current conditions, 7-day forecasts, and hourly forecasts, powered by the [Open-Meteo API](https://open-meteo.com).

## Install

### Homebrew

```sh
brew tap magichrist/tap
brew install weather_cli
```

### Build from source

```sh
git clone https://github.com/magichrist/weather_cli
cd weather_cli
cargo build --release
```

## Usage

```
weather_cli [OPTIONS]

Options:
  -a <A>             Latitude (requires -b)
  -b <B>             Longitude (requires -a)
  -s, --search <CITY>  Search by city name
      --ml           Use IP-based geolocation
  -c, --clear_cache  Clear the local cache
  -i, --interactive  Interactive mode
  -f, --forecast     Show 7-day forecast instead of current weather
      --hourly [DAYS]  Hourly forecast (default: 3 days, max: 16)
      --json         Output raw JSON
  -h, --help         Print help
  -V, --version      Print version
```

### Examples

```sh
# Search by city name
weather_cli -s Paris
weather_cli -s "New York" -f

# Current weather by coordinates
weather_cli -a 48.8566 -b 2.3522

# 7-day forecast for New York
weather_cli -a 40.7128 -b -74.0060 -f

# Hourly forecast (3 days)
weather_cli -a 48.8566 -b 2.3522 --hourly

# Hourly forecast (7 days)
weather_cli -a 48.8566 -b 2.3522 --hourly 7

# City search + hourly
weather_cli -s Tokyo --hourly

# Auto-detect location from IP
weather_cli --ml

# Output as JSON (for scripting)
weather_cli -a 48.8566 -b 2.3522 --json
weather_cli -s London -f --json

# Interactive REPL
weather_cli -i
```

### Interactive mode

Type `LAT LON` pairs, `help` for commands, or `q` to exit:

```
$ 48.85 2.35
Current Weather Data:
Location: 48.8566, 2.3522
...

$ help
Commands:
  <LAT> <LON> — enter coordinates as LAT LON
  help — show this help
  q — exit

$ q
```

## Architecture

```
src/
├── main.rs      — CLI parsing and dispatch
├── app.rs       — Business logic (direct, interactive, geolocation, search)
├── api.rs       — HTTP fetching with Open-Meteo API + geocoding
├── cache.rs     — On-disk cache ($XDG_CACHE_HOME/weather_cli/)
├── models.rs    — API response types, WMO weather code mapping
├── display.rs   — Terminal output (tables, charts, hourly grid)
└── error.rs     — Typed error handling (AppError + AppResult)
```

## Features

- **City name search** — find locations by name via Open-Meteo Geocoding API (`-s`)
- **Enhanced current weather** — temperature, feels-like, humidity, pressure, wind, UV index, rain/snow
- **Weather condition text** — human-readable WMO codes (Clear sky, Rain, Snow, Thunderstorm, etc.)
- **7-day forecast table** with temperature chart (`textplots`)
- **Hourly forecast** — hourly temp, precipitation %, and conditions (`--hourly`)
- **JSON output** — raw API response for scripting and pipelines (`--json`)
- **Colored output** — terminal-colored labels for weather fields and forecast tables
- **On-disk cache** — responses cached for 5 hours under `$XDG_CACHE_HOME/weather_cli/`
- **IP geolocation** — auto-detect location via HTTPS IP lookup (`--ml`)
- **Typed errors** — `thiserror`-based `AppError` enum
- **HTTP timeout** — 15-second timeout prevents infinite hangs on network issues
- **CI** — GitHub Actions runs `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test`

## Development

```sh
# Run tests
cargo test

# Lint
cargo clippy --all-targets --all-features

# Format
cargo fmt

# Debug logging
RUST_LOG=debug cargo run -- -a 48.85 -b 2.35
```

## License

MIT
