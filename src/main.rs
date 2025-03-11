use serde_json::{from_str};
use serde::{Deserialize};
use structopt::StructOpt;
use reqwest;
use reqwest::Url;

/*
Example API Response:
```json
{
  "current": {
    "time": "2025-03-11T05:45",
    "interval": 900,
    "temperature_2m": 2.2
  }
}
Error Response:

json
Copy
Edit
{
  "error": true,
  "reason": "Invalid input parameters"
}
*/

#[derive(StructOpt)]
struct Parameters { 
  longitude: f64,
  latitude: f64,
}

#[derive(Deserialize, Debug)]
struct Failed { 
  error: bool,
  reason: String,
}

#[derive(Deserialize, Debug)]
struct DataValues {
  time: String,
  #[serde(rename = "temperature_2m")]
  temperature: f64,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
  current: DataValues,
}

#[tokio::main]
async fn main() { 
  let params = Parameters::from_args();
  println!( "Params: latitude -> {:?}; longitude -> {:?}", params.latitude, params.longitude );

  let url = format!(
    "https://api.open-meteo.com/v1/jma?latitude={}&longitude={}&current=temperature_2m",
    params.latitude,
    params.longitude
  );
  println!("Formatted URL: {}", url);

  match Url::parse(&url) {
    Ok(urlparsed) => match reqwest::get(urlparsed).await {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap(); // Convert response to text

            if status.is_client_error() {
                // Try parsing as error response
                match from_str::<Failed>(&body) {
                    Ok(error_response) => eprintln!("Error 400++: {:?}", error_response),
                    Err(_) => eprintln!("Failed to parse API error response"),
                }
            } else {
                // Try parsing as weather response
                match from_str::<WeatherResponse>(&body) {
                    Ok(weather) => println!("Success 200++: {:?}", weather.current),
                    Err(e) => eprintln!("Failed to parse weather response: {:?}", e),
                }
            }
        }
        Err(e) => eprintln!("Error occurred while requesting API: {:?}", e),
    },
    Err(e) => eprintln!("Error parsing URL: {:?}", e),
  }
}
