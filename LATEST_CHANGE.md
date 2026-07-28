# Changelog

All notable changes to this project.

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
