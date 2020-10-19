/// API key
use serde_json;
use structopt::StructOpt;

#[structopt(name = "weather", about = "Get today's weather details for the City.")]
#[derive(Debug, StructOpt)]
pub struct Opt {
    /// City to check Weather
    city: String,
    /// Country Code
    #[structopt(default_value = "IN")]
    country: String,
}

/// Struct for weather data fetched
#[derive(Debug, Clone)]
pub struct Data {
    city: String,
    country: String,
    coord: (f32, f32),
    min_temp: f32,
    max_temp: f32,
    temp: f32,
    feels_like: f32,
    humidity: u32,
    wind_speed: f32,
    wind_dir: String,
}
impl Data {
    /// Get a Data struct with all the attributes described
    pub fn from_json(records: serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        
        let city: String = records["name"].to_string();
        let country: String = records["sys"]["country"].to_string();
        let coord: (f32, f32) = {
            let c = &records["coord"];
            (c["lat"].to_string().parse()?, c["lon"].to_string().parse()?)
        };

        let _m = &records["main"];
        let min_temp: f32 = _m["temp_min"].to_string().parse()?;
        let max_temp: f32 = _m["temp_max"].to_string().parse()?;
        let temp: f32 = _m["temp"].to_string().parse()?;
        let feels_like: f32 = _m["feels_like"].to_string().parse()?;
        let humidity: u32 = _m["humidity"].to_string().parse()?;

        let _w = &records["wind"];
        let wind_speed: f32 = _w["speed"].to_string().parse()?;
        // Mapping wind to named directions like N,NE.. 
        let wind_dir = {
            let _dir: i32 = _w["deg"].to_string().parse()?;
            let _d = {
                vec![0i32, 1, 2, 3, 4, 5, 6, 7].iter().fold(420, |acc, &x| {
                    let d1: i32 = x * 45 - _dir;
                    let d2: i32 = acc * 54 - _dir;
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

/// Makes request and returns Data
pub mod req {
    use crate::*;
    use reqwest;
    const APP_ID: &str = "14da5e4cac40d2e8893248d960ce48b6";

    pub fn make_request(cli: Opt) -> Result<Data, Box<dyn std::error::Error>> {
        let req_msg = format! {"http://api.openweathermap.org/data/2.5/weather?q={},{},{}&appid={}&units=metric", cli.city , "0", cli.country ,APP_ID};
        //.unwrap_or_else(| | "IN".to_string())
        let body = reqwest::blocking::get(&req_msg)?.text()?;
        Data::from_json(serde_json::from_str(&body)?)
    }
}
