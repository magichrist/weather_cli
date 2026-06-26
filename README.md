[![Test&Release](https://github.com/magichrist/weather_cli/actions/workflows/test.yml/badge.svg)](https://github.com/magichrist/weather_cli/actions/workflows/test.yml)
[![CodeQL](https://github.com/magichrist/weather_cli/actions/workflows/codeql.yml/badge.svg)](https://github.com/magichrist/weather_cli/actions/workflows/codeql.yml)
<br>
[![Release](https://img.shields.io/github/v/release/magichrist/weather_cli)](https://github.com/magichrist/weather_cli/releases)
[![Downloads](https://img.shields.io/github/downloads/magichrist/weather_cli/total)](https://github.com/magichrist/weather_cli/releases)
[![Rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust)](https://www.rust-lang.org/)


# A weather cli tool
To get weather current state and forcast of 7 days
**it works with with lat lon to get data**

## Build
Easy as F
### Get
Either get the built binary or build it

#### Get from homebrew
```
brew tap magichrist/tap
brew install weather_cli
```

#### Build
**Get the repo and install dependencies**
```
git clone https://github.com/magichrist/weather_cli
cd weather_cli
```
**build it**
```
just b
```

## Usage
```
Usage: weather_cli [OPTIONS]

Options:
  -a <A>             Lat (must be used with -b)
  -b <B>             Lon (must be used with -a)
      --ml           mylocation
  -c, --clear_cache  Clear Cache
  -i, --interactive  Interactive mode
  -f, --forecast     Forecast mode
  -h, --help         Print help
  -V, --version      Print version
```
