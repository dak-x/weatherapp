// ! Currently add support for only 1 option i.e. city name. Later continue to add more functionalities.

use weather::*;
use structopt::StructOpt;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let c = Opt::from_args();
    println!("{:?}",req::make_request(c)?);
    Ok(())
}
