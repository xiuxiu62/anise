#![allow(dead_code)]
#![feature(format_args_capture)]

mod client;
mod error;
mod options;
mod show;

use client::Client;
use error::AniseResult;
use options::Options;

use structopt::StructOpt;

fn main() -> AniseResult<()> {
    let options = Options::from_args();
    let client = Client::new(options);
    let response = client.search_anime("violet evergarden")?;
    response.iter().for_each(|show| println!("{show:?}"));

    let response = client.search_anime("naruto")?;
    response.iter().for_each(|show| println!("{show:?}"));

    let response = client.search_anime("akame")?;
    response.iter().for_each(|show| println!("{show:?}"));

    Ok(())
}
