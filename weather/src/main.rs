mod forecast;
use forecast::Forecast;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    api_key: String,
    city: String,
    country_code: String,
}

#[tokio::main]
async fn main() {
    // parse args
    let args = Cli::from_args();

    // get from api
    let reponse = Forecast::get(args.api_key, args.city, args.country_code)
        .await
        .unwrap();

    // simple display
    println!("Weather: {}", reponse.weather.details.description);
    println!("Temperature: {}°C", reponse.main.temp);
    println!("Wind Speed: {}m/s", reponse.wind.speed);
}
