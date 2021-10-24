#![allow(dead_code)]

mod client;
mod options;

use client::Client;
use options::Options;

use structopt::StructOpt;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::from_args();
    let client = Client::new(options);
    let response = client.search_anime("violet evergarden").await?;

    Ok(())
}
