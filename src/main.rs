#![allow(dead_code,unused)]

use weather::*;
use structopt::StructOpt;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut c = Weather::from_args();
    c.make_request()?;
    println!("{}",c);

    Ok(())
}