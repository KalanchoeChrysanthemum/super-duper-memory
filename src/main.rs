extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref _prog = args[0];

    let mut opts = Options::new();

    // Opt flags
    opts.optflag("h", "hello", "prints hello world");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string());
        }
    };

    if matches.opt_present("h") {
        println!("Hello, World!");
    }
}
