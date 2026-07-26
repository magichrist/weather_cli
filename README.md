# weather_cli

A weather CLI tool for current conditions and 7-day forecasts, powered by the [Open-Meteo API](https://open-meteo.com).

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
      --ml           Use IP-based geolocation
  -c, --clear_cache  Clear the local cache
  -i, --interactive  Interactive mode
  -f, --forecast     Show 7-day forecast instead of current weather
  -h, --help         Print help
  -V, --version      Print version
```

### Examples

```sh
# Current weather in Paris
weather_cli -a 48.8566 -b 2.3522

# 7-day forecast for New York
weather_cli -a 40.7128 -b -74.0060 -f

# Auto-detect location from IP
weather_cli --ml

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
├── app.rs       — Business logic (direct, interactive, geolocation)
├── api.rs       — HTTP fetching with Open-Meteo API
├── cache.rs     — On-disk cache ($XDG_CACHE_HOME/weather_cli/)
├── models.rs    — API response types and serialization
├── display.rs   — Terminal output (tables and charts)
└── error.rs     — Typed error handling (AppError + AppResult)
```

## Features

- **Colored output** — terminal-colored labels for weather fields and forecast tables
- **On-disk cache** — responses cached for 5 hours under `$XDG_CACHE_HOME/weather_cli/`
- **IP geolocation** — auto-detect location via HTTPS IP lookup (`--ml`)
- **7-day forecast table** with temperature chart (`textplots`)
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
