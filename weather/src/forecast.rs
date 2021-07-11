use exitfailure::ExitFailure;
use reqwest::Url;
use serde_derive::{Deserialize, Serialize};

// thanks to https://stackoverflow.com/questions/53866508/how-to-make-a-public-struct-where-all-fields-are-public-without-repeating-pub
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty),* $(,)?}) => {
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(Forecast {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Temps,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
});

impl Forecast {
    pub async fn get(
        api_key: String,
        city: String,
        country_code: String,
    ) -> Result<Self, ExitFailure> {
        // format url
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
            city, country_code, api_key
        );
        let url = Url::parse(&*url).unwrap();
        // get page and deserialize
        let response = reqwest::get(url)
            .await
            .unwrap()
            .json::<Forecast>()
            .await
            .unwrap();
        Ok(response)
    }
}

pub_struct!(Coord { lon: f64, lat: f64 });

pub_struct!(Weather { details: Details });

pub_struct!(Details {
    id: i32,
    main: String,
    description: String,
    icon: String,
});

pub_struct!(Temps {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
});

pub_struct!(Wind {
    speed: f64,
    deg: i32,
});

pub_struct!(Clouds { all: i32 });

pub_struct!(Sys {
    r#type: f64,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
});
