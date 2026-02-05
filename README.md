# An weather cli tool
To get weather current state and forcast of 7 days
**it works with with lat lon to get data**

## Build
Easy as F
### Get
either get the built binary or build it
### Build
**Get the repo and install dependencies**
```
git clone https://github.com/magichrist/weather_cli
cd weather_cli
cargo run 
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
