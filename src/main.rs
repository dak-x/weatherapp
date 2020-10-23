// ! Currently add support for only 1 option i.e. city name. Later continue to add more functionalities.

use weather::*;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c = Weather::from_args();
    c.make_request()?;
    println!("{}",c);

    Ok(())
}