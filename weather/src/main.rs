mod forecast;
use forecast::Forecast;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    city: String,
    country_code: String,
}

#[tokio::main]
async fn main() {
    // parse args
    let args = Cli::from_args();

    let api_key = env::var("API_KEY").expect("Env var API_KEY is required.");

    let response = Forecast::get(api_key, args.city, args.country_code)
        .await
        .unwrap();

    // simple display
    println!("Weather: {}", response.weather.details.description);
    println!("Temperature: {}°C", response.main.temp);
    println!("Wind Speed: {}m/s", response.wind.speed);
}
