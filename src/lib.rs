/// API key
pub const APP_ID: &str = "14da5e4cac40d2e8893248d960ce48b6";
use serde_json::{self, Map, Value};
use std::fmt::{self, Display};

#[derive(Debug, Clone)]
pub struct WeatherData {
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

impl WeatherData {
    // Get a weatherData struct with all the attributes described above
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
        Ok(WeatherData {
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
