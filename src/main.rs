use std::error::Error;

use configurator::configurator::{build_config, parse_args};
use getopts::Matches;

extern crate getopts;


mod configurator;

fn main() -> Result<(), Box<dyn Error>>{

    let flags: Matches = parse_args()?;

    let conf = build_config(flags);

    println!("{:?}", conf);

    Ok(())
}
