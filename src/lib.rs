#![allow(dead_code, unused)]

use serde_json;
use structopt::StructOpt;
use anyhow::Result;

#[structopt(name = "weather", about = "Get today's weather details for the City.")]
#[derive(Debug, StructOpt)]
pub struct Weather {
    #[structopt(subcommand)]
    commands: Cmds,
    /// Set for More Info.
    #[structopt(short, long)]
    details: bool,

    #[structopt(skip)]
    data: Option<Data>,

    #[structopt(skip)]
    error: Option<String>,
}

#[derive(StructOpt, Debug)]
enum Cmds {
    /// Get weather details of the city
    City(CityArgs),
    /// Get weather details from map coordinates
    Coords(CoordArgs),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "City", about = "Get weather details of the city")]
struct CityArgs {
    /// City to check Weather
    city: String,
    /// Country Code
    #[structopt(requires("city"), default_value = "IN")]
    country: String,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Coordinates",
    about = "Get weather details from map coordinates"
)]
struct CoordArgs {
    /// Specify Latitude
    lat: f64,
    /// Specify Longitude
    long: f64,
}


/// Struct for weather data fetched
#[derive(Debug, Clone)]
pub struct Data {
    city: String,
    country: String,
    coord: (f64, f64),
    min_temp: f64,
    max_temp: f64,
    temp: f64,
    feels_like: f64,
    humidity: u64,
    wind_speed: f64,
    wind_dir: String,
}
impl Data {
    /// Get a Data struct with all the attributes described
    pub fn from_json(records: serde_json::Value) -> Result<Self> {
        let city: String = records["name"].to_string();
        let country: String = records["sys"]["country"].to_string();
        let coord: (f64, f64) = {
            let c = &records["coord"];
            (
                c["lat"].as_f64().unwrap_or_else(|| 0.0),
                c["lon"].as_f64().unwrap_or_else(|| 0.0),
            )
        };

        let _m = &records["main"];
        let min_temp: f64 = _m["temp_min"].as_f64().unwrap_or_else(|| 0.0);
        let max_temp: f64 = _m["temp_max"].as_f64().unwrap_or_else(|| 0.0);
        let temp: f64 = _m["temp"].as_f64().unwrap_or_else(|| 0.0);
        let feels_like: f64 = _m["feels_like"].as_f64().unwrap_or_else(|| 0.0);
        let humidity: u64 = _m["humidity"].as_u64().unwrap_or_else(|| 0);

        let _w = &records["wind"];
        let wind_speed: f64 = _w["speed"].as_f64().unwrap_or_else(|| 0.0);
        // Mapping wind to named directions like N,NE..
        let wind_dir = {
            let _dir: f64 = _w["deg"].to_string().parse()?;
            let _d = {
                vec![0f64, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]
                    .iter()
                    .fold(420.0, |acc, &x| {
                        let d1: f64 = x * 45.0 - _dir;
                        let d2: f64 = acc * 45.0 - _dir;
                        if d1.abs() < d2.abs() {
                            x
                        } else {
                            acc
                        }
                    })
            };
            vec!["N", "NE", "E", "SE", "S", "SW", "W", "NW"][_d as usize].to_string()
        };
        Ok(Data {
            city,
            country,
            coord,
            min_temp,
            max_temp,
            temp,
            feels_like,
            humidity,
            wind_speed,
            wind_dir,
        })
    }
}

impl std::fmt::Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use colored::Colorize;

        if self.data.is_some() {
            //checked above so unwrap is fine
            let data = self.data.as_ref().unwrap();

            let color = |s: String| match data.temp {
                x if x < 20.0 => s.bright_cyan(),
                x if x < 30.0 => s.bright_yellow(),
                _ => s.bright_red(),
            };
            let fmt_string = |s: String| {
                let l = s.len();
                s[1..l - 1].to_string()
            };

            writeln! {f, ""};
            writeln! {f, " {} {}         {}: {}{}",
                      color(fmt_string(data.city.clone())), color(fmt_string(data.country.clone())), "Feels Like".yellow(), color(data.feels_like.to_string()), color("°C".to_string())};

            writeln! {f, " {}{}{}    {}{}{}",
                      "Temp: ".yellow(), color(data.temp.to_string()), color("°C".to_string()),
                      "Humidity: ".blue(), data.humidity.to_string().bright_blue(), "%".bright_blue()
            };

            writeln! {f, " {} {}{}       {}{}{}",
                      "Min: ".cyan(), data.min_temp.to_string().cyan(), "°C".cyan(),
                      "Max: ".red(), data.max_temp.to_string().red(), "°C".red()
            };

            // Detailed flag is set then diaply all the stuff
            if self.details {
                writeln! {f, " {}{}       {}{}",
                          "Lat: ".bright_green(), data.coord.0.to_string().bright_green(),
                          "Long: ".bright_green(), data.coord.1.to_string().bright_green(),
                };
                writeln! {f, " {}{} {}", "Wind: ".bright_blue(),
                          data.wind_speed.to_string().bright_yellow(),
                          data.wind_dir.bright_red().bold()
                };
            }

            writeln! {f, ""}
        } else if self.error.is_some() {
            writeln! {f, "Request failed: {}", self.error.as_ref().unwrap()}
        } else {
            panic!("Request Not Done");
        }
    }
}

/// Makes request and returns Data
pub mod req {
    use crate::*;
    use reqwest;
    use anyhow::Context;

    const REQ: &str = "http://api.openweathermap.org/data/2.5/weather?";

    impl Weather {
        pub fn make_request(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            use std::env;

            let APP_ID: &'static str = env!("OPENWEATHERAPI", "API token not found");
            let req_msg: String = match &self.commands {
                Cmds::City(city_args) => {
                    format! {"{}q={},{},{}&appid={}",REQ, city_args.city, "0" ,city_args.country, APP_ID }
                }
                Cmds::Coords(coord_args) => {
                    format! {"{}lat={}&lon={}&appid={}",REQ, coord_args.lat,coord_args.long,APP_ID}
                }
            };

            let req_msg = format! {"{}&units={}",req_msg,"metric"};
            let body = reqwest::blocking::get(&req_msg).context("OpenWeather request failed")?.text()?;
            self.data = match Data::from_json(serde_json::from_str(&body)?)
            {
                Ok(data) => {
                    self.error = None;
                    Some(data)
                },
                Err(e) => {
                    self.error = Some(serde_json::from_str::<serde_json::Value>(&body)?["message"].to_string());
                    None
                }
            };
            Ok(())
        }
    }
}
