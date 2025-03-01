extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let ref _prog = args[0];

    let mut opts = Options::new();

    // Opt flags
    opts.optflag("h", "hello", "prints hello world");
    opts.optflag("t", "time", "Benchmarks time");
    opts.optflag("m", "memory", "Benchmarks memory");
    opts.optflag("d", "disk", "Benchmarks disk usage");
    opts.optflag("r", "ram", "Benchmarks RAM usage");
    opts.optflag("c", "cpu", "Benchmarks CPU usage");
    opts.optflag("g", "gpu", "Benchmarks GPU usage");
    opts.optflag("p", "processes", "something children processes?");
    opts.optflag("s", "sys", "Track all syscalls");
    opts.optflag("f", "full", "Benchmark everything")



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
