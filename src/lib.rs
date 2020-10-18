/// API key
pub const APP_ID: &str = "14da5e4cac40d2e8893248d960ce48b6";
use serde::{Deserialize, Serialize};
use serde_json::{self,Value};
use std::fmt::{self, Display};

/// Structure to capture all the use information about the weather
#[derive(Serialize, Deserialize)]
pub struct WeatherData {
    city: String,
    country: String,
    coord: (f32, f32),
    min_temp: f32,
    max_temp: f32,
    temp: f32,
    feels_like: f32,
    humidity: u32,
    wind_speed: u32,
    wind_dir: String,
}

#[derive(Debug)]
struct ParseError {
    error: String,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Coundn't find field {}", self.error)
    }
}
impl std::error::Error for ParseError {
    // Already defaulted
}

enum Jsonval {
    val(serde_json::Value),
}

impl Jsonval {
    fn get_bool(&self) -> Result<bool, ParseError> {
        match &self {
            Jsonval::val(serde_json::Value::Bool(b)) => Ok(*b),
            _ => Err(ParseError {
                error: format!("Cannot Find Bool"),
            }),
        }
    }

    fn get_number(&self) -> Result<f64,ParseError>{
        match &self {
            Jsonval::val(serde_json::Value::Number(x)) => Ok(x.as_f64().unwrap()),
            _ => Err(ParseError{
                error: format!("Cannot Find Number"),
            })
        }
    }

    fn get_String(&self) -> Result<String, ParseError>{
        match &self {
            Jsonval::val(serde_json::Value::String(s)) => Ok(s.to_string()),
            _ => Err(ParseError{
                error: format!("Cannot Find String"),
            })
            
        }       
    }

    fn get_map(&self) -> Result<&serde_json::Map<String,Value>, ParseError>{

        match &self {
            Jsonval::val(serde_json::Value::Object(x)) => Ok(x), 
            _ => Err(ParseError{
                error: format!("Cannot Find Record"),
            })
        }
    }
}
