# source weather API:
[Open meteo: Free No API KEY Needed If Non Commercial Use](https://open-meteo.com/en/docs/jma-api?latitude=35.6895&longitude=139.6917&current=temperature_2m)

- Request: https://api.open-meteo.com/v1/jma?&latitude=52.52&longitude=13.41&current=temperature_2m
- Response `Ok()`:
```bash
{
  "latitude": 52.5,
  "longitude": 13.5,
  "generationtime_ms": 0.00965595245361328,
  "utc_offset_seconds": 0,
  "timezone": "GMT",
  "timezone_abbreviation": "GMT",
  "elevation": 38,
  "current_units": {
    "time": "iso8601",
    "interval": "seconds",
    "temperature_2m": "°C"
  },
  "current": {
    "time": "2025-03-11T05:45",
    "interval": 900,
    "temperature_2m": 2.2
  }
}
```
- Response `Err()`:
```bash
{
  "error": true, 
  "reason": "Cannot initialize WeatherVariable from invalid String value
   tempeture_2m for key hourly" 
}
```

- Structure url:
https://api.open-meteo.com/v1/jma?&latitude=52.52&longitude=13.41&current=temperature_2m
url_schema:`<url>`			`?` 	`&<location>`		`&<requested data key=value>`

## Plan:

### Need to send:
- float values: `longitude`, `latitude`
- string value equal to: `temperature_2m`
`https://api.open-meteo.com/v1/jma?&latitude=<latitude_float_value>&longitude=<longitude_float_value>&current=<string_temperature_2m>`

### Need response extraction:
```bash
{
  "current": {
    "time": "2025-03-11T05:45",
    "interval": 900,
    "temperature_2m": 2.2
  }
}
```
- String: `current.time`
- Float: `current.temperature_2m`

# How To Use:
- use from CLI 
```bash
cargo run <latitude> <longitude>
```
- Output
You will get the temperature

# Code Improvement
- [] Accept some more parameter
- [] Accept parameters that are also picking on the type of data desired
- [] Improve, therefore, structs to be more permissive using `generic` `Types<T>`
