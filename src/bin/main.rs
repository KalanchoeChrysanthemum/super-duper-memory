extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
    println!("[INFO] Running main.rs");

    let args: Vec<String> = env::args().collect();
    let ref _prog = args[0];

    let mut flags = Options::new();
    set_flags(&mut flags);

    let opts = match flags.parse(&args[1..]) {
        Ok(o) => o,
        Err(e) => {
            panic!("[ERROR] {}", e.to_string());
        }
    };

    if opts.opt_present("test") {
        temp_test(&opts.opt_str("test").unwrap());
    }
}

// Temporary function for testing purposes
// put (program under test)
fn temp_test(put: &str) {
    println!("[INFO] Test ran with '{}'", put);
}

fn set_flags(flags: &mut Options) {
    // Temp flag for testing purposes
    flags.optopt("", "test", "placeholder testing flag", "EXE");

    flags.optflag("t", "time", "Benchmarks time");
    flags.optflag("m", "memory", "Benchmarks memory");
    flags.optflag("d", "disk", "Benchmarks disk usage");
    flags.optflag("r", "ram", "Benchmarks RAM usage");
    flags.optflag("c", "cpu", "Benchmarks CPU usage");
    flags.optflag("g", "gpu", "Benchmarks GPU usage");
    flags.optflag("p", "processes", "something children processes?");
    flags.optflag("s", "sys", "Track all syscalls");
    flags.optflag("f", "full", "Benchmark everything");
}
