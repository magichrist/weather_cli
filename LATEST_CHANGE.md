# Changelog

All notable changes to this project.

## v0.5.0 - 2026-07-28



- 28636ca docs: update README with features section and bug fixes

- d2ba265 feat: city search, enhanced weather, hourly forecast, JSON output, cache fixes

- City name search via Open-Meteo Geocoding API (-s/--search)
- Enhanced current weather: humidity, UV index, pressure, feels-like, condition text
- WMO weather code to human-readable text mapping
- Hourly forecast (--hourly [DAYS], default 3)
- JSON output mode (--json) for scripting
- Fix --json --ml and --ml --hourly (dispatch now resolves coords from any source)
- Fix --json bypassing cache (now checks and stores cache)
- 23 tests passing, clippy clean

- 103e38b fix: --hourly --json now returns JSON; add Rust cache to CodeQL workflow

- handle_hourly now accepts json flag for JSON output
- Rust cache added to codeql.yml to avoid recompiling on every run

- 0b137c6 fix: city search --hourly --json now returns hourly JSON

- In handle_city_search, check hourly before json to avoid early return
- Suppress 'Selected:' label when --json is set to keep output clean

- d9e26fd Version v0.5.0, added many features
