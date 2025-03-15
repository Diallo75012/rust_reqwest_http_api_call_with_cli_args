use serde_json::{from_str};
use serde::{Deserialize};
use structopt::StructOpt;
use reqwest;
use reqwest::Url;
// this to check types as not using advanced IDE
use std::any::type_name;


fn print_type<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}
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

// here `StructOpt` `Struct` will help us to fetch `CLI` input parameters
// in the `CLI` input parameters will be separated by spaces
#[derive(StructOpt)]
struct Parameters { 
  longitude: f64,
  latitude: f64,
}

// we want to catch the reason of the response error
// we checked the structure of it and match it here
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Failed { 
  error: bool,
  reason: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct DataValues {
  time: String,
  // here the name that we have defined for this field is different from
  // the one returned by the `API` so we use `serde(rename...)` to match it
  // and not get error
  #[serde(rename = "temperature_2m")]
  temperature: f64,
  // we could also use the matching name field returned by the `API` direclty
  // temperature_2m: f64,
}

// here we match the `API` response by just getting the fields
// that we need to fetch from it, as it returns more
// but here we will use the `serde` `Deserialization` laziness at our advantage
#[derive(Deserialize, Debug)]
struct WeatherResponse {
  current: DataValues,
}

// always `tokio` because we love `Japan`
// it also serves as `async` implementer (`async`/`await`) on the `main` function
#[tokio::main]
async fn main() { 
  // we get the parameters from the `CLI` input parameters
  // `Paramters` struct derives `StructOpt` so we can use `from_args()` method
  let params = Parameters::from_args();
  println!( "Params: latitude -> {:?}; longitude -> {:?}", params.latitude, params.longitude );

  // we format the url to take in those `CLI` input parameters
  let url = format!(
    "https://api.open-meteo.com/v1/jma?latitude={}&longitude={}&current=temperature_2m",
    // `params` is of type `Parameter` so we can access field using `.` (dot) notation
    params.latitude,
    params.longitude
  );
  println!("Formatted URL: {}", url);

  // example of using lifetime short to drop the var if don't want to use it after
  // even if here we could have used it for the match pattern. just playing with it...
  {
    let url_type_check = Url::parse(&url);
    print_type(&url_type_check);
  }
  // I try to not use `unwrap` and it helps me to understand what I am doing
  // and to get more errors and fix those and understand even more
  // we match on the parsing of the Url as we need to check every step
  // and manage what could happen if it happens
  // Result<Url, url::ParseError>
  match Url::parse(&url) {
    // Route `url` parsed `Ok()`, we pass it to `request` and match on `await` of it
    // `response` or not?
    // `reqwest::get(urlparsed)`: `impl Future<Output = Result<Response, reqwest::Error>>`
    // therefore, does nothing, need `.await` to become: `Result<Response, reqwest::Error>`
    Ok(urlparsed) => match reqwest::get(urlparsed).await {
        // `response` Ok(), we are going to check if we got an error response or a success one
        Ok(response) => {
            // we get the `status code`
            // `status`: `reqwest::StatusCode`
            let status = response.status();
            print_type(&status);

            // we get the `body` of the response (could be `error...because` or `success...`)
            // `Response.text()`:  `impl Future<Output = Result<Response, reqwest::Error>>`
            // therefore, does nothing, need `.await` to become: `Result<Response, reqwest::Error>`
            // let body = response.text().await.unwrap(); // Convert response to text
            // print_type(&body);

            match response.text().await {
              Ok(body) => {
                print_type(&body);
                // HERE we check the status code. Could have used the other way around with :`.is_success()`
                if status.is_client_error() {
                  // Try parsing as error response using match
                  match from_str::<Failed>(&body) {
                    // is our defined custom struct `Deserialize` it fine -> `Ok()` or `Err()`
                    Ok(error_response) => eprintln!("Error 400++: {:?}", error_response),
                    Err(_) => eprintln!("Failed to parse API error response"),
                  }
                } else {
                  // Try parsing as weather response as no error in `status code`
                  // we now match to check if the response can be `Deserialized` by our custom `struct`
                  match from_str::<WeatherResponse>(&body) {
                    // Ok()? then we return the weather forecast! Or `Err()` we return error
                    Ok(weather) => println!("Success 200++: {:?}", weather.current),
                    Err(e) => eprintln!("Failed to parse weather response: {:?}", e),
                  }
                }
              },
              Err(e) => eprintln!("Failed to read response text: {:?}", e),
            }
        },
        Err(e) => eprintln!("Error occurred while requesting API: {:?}", e),
    },
    Err(e) => eprintln!("Error parsing URL: {:?}", e),
  }
}
