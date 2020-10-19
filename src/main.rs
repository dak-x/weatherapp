// ! Currently add support for only 1 option i.e. city name. Later continue to add more functionalities.

use serde_json::Value;
use reqwest::{blocking, Error};
use structopt::StructOpt;
use wethrapp::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "Weather", about = "Get weather details of the specified City.")]
struct Opt {
    /// City to get weather data
    city: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = "Delhi";
    let req = format! {"http://api.openweathermap.org/data/2.5/weather?q={},{},{}&appid={}&units=metric", city , "0", "IN",APP_ID};
    
    let body = reqwest::blocking::get(&req)?.text()?;
    let json: Value = serde_json::from_str(&body)?;
    println!("{:?}",WeatherData::from_json(json));
    Ok(())
}
