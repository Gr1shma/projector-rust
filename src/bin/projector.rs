use clap::Parser;
use projector_rust::{config::Config, opts::Opts};
use anyhow::Result;

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(());
}

