#![allow(dead_code)]
#![feature(format_args_capture)]

mod client;
mod options;

use client::Client;
use options::Options;

use structopt::StructOpt;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();
    let client = Client::new(options);
    let response = client.search_anime("violet evergarden")?;
    response.iter().for_each(|t| println!("{t}"));

    let response = client.search_anime("naruto")?;
    response.iter().for_each(|t| println!("{t}"));

    Ok(())
}
