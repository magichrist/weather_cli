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

## v0.4.0 - 2026-07-26



- 94f40df not expected behaviour from cliff; fixed

- c9316e7 workflow update

- d3f411e workflow update

- ce4944c fix(lint): fixed some lints

- 989717f feat(precommit): fmt, betterleaks added

- 593f2f1 chore(precommit): changelog

- 1958879 fix(clippy): resolved clippy crys

- eb54814 fix: full codebase cleanup — clippy warnings, panics, dead code, type consistency

- Remove all #[allow(dead_code)] and #[allow(clippy::...)] suppressions
- Fix 5 clippy warnings (&Box<T>, unwrap after is_some, collapsible_if, manual_range_contains)
- Replace all unwrap()/expect() with graceful error handling
- Unify float types: Current fields f32 → f64 (matches Daily)
- Extract calc_and_fetch params instead of re-parsing Args::parse()
- Deduplicate depictors match arms, fix hardcoded chart range
- Fix cache underflow with checked_sub, remove dead code and stale comments

- 676e0d7 COMPLETE REFACTOR

- 99bf6de fix: address 8 bugs found by bug hunter audit

- BUG-001: Add 15s HTTP request timeout via shared reqwest::Client
- BUG-002: parse_coords now shows actual invalid input in error
- BUG-003: Switch IP geolocation to HTTPS
- BUG-004: Guard depict_forecast against empty daily data
- BUG-005: Cache::persist logs write errors via debug!()
- BUG-006: Cache::load logs parse/read errors via debug!()
- BUG-007: Remove dead Display impl for WeatherResponse
- BUG-008: Eliminate double Cache::load in --ml path

- 1d5413a fix: address 8 bugs found by bug hunter audit

- BUG-001: Add 15s HTTP request timeout via shared reqwest::Client
- BUG-002: parse_coords now shows actual invalid input in error
- BUG-003: Switch IP geolocation to HTTPS
- BUG-004: Guard depict_forecast against empty daily data
- BUG-005: Cache::persist logs write errors via debug!()
- BUG-006: Cache::load logs parse/read errors via debug!()
- BUG-007: Remove dead Display impl for WeatherResponse
- BUG-008: Eliminate double Cache::load in --ml path

- 11ca126 Merge remote-tracking branch 'refs/remotes/origin/main'

## v0.3.0 - 2026-06-26



- 2c6c400 added cli-spinner

- 911e24a added cli-spinner

- e0855bd details for spinner

- c0d8a69 fmt

- 03766a9 better ui for forecast

- d983410 Update test.yml

no need to use gunzip

- 0604013 Create codeql.yml

- 9c68d57 version v0.3.0

- ac4019b version v0.3.0

## v0.2.6 - 2026-04-28



- eb47d43 better code

- 4758bbc docs

- cc65392 better code

- 8f3f857 version 0.2.6

## v0.2.5 - 2026-02-11



- 6d85fc6 forecast double printing bug fixed

- a559790 workflow

- 5dbde4f workflow

- 13f870c added brew

- 23b2373 added webhook

## v0.2.4 - 2026-02-07



- d585888 Initial commit

- bceff8f Initial commit

- feec177 Initial commit

- 7963ee1 automated release and fixed bugs

- 538c17d workflow

- 3ecd518 workflow

- ddc77b5 workflow

- 52849c0 automated release

- 9024243 automated release

- a050cf8 automated release

- fe297ee automated release

- d44ee70 automated release

- 249583d version 0.2.3

- 9a832b5 version 0.2.3

- 4783b0b workflow
