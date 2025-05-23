// TODO!
//
// Testing / Dev:
//
// Create and verify sample programs to benchmark each component
// (GPU, CPU, RAM, etc)
//
// Create sample methods to run aforementioned benchmark programs
//
// Add test flag to run these programs to demo app ??
//
// Add support for test function to execute commands
//
//
//
// Production (Assumes above is done / partially done):
//
// ! Not everything left, just what needs/can be done now !
//
// Add graphing functionality to view results




use getopts::Options;
use colored::{ColoredString, Colorize};

use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;


// Flag for printing debug info
static VERBOSE: OnceLock<AtomicBool> = OnceLock::new();


#[allow(dead_code)]
enum LogType {
    INFO,
    WARN,
    ERROR,
}

impl LogType {
    fn label(&self) -> ColoredString {
        match self {
            LogType::INFO => "[INFO]".blue().bold(),
            LogType::WARN => "[WARN]".yellow().bold(),
            LogType::ERROR => "[ERROR]".red().bold(),
        }
    }
}

fn is_verbose() -> bool {
    VERBOSE
        .get()
        .map(|v| v.load(Ordering::Relaxed))
        .unwrap_or(false)
}


//==============================
//          MACROS
//==============================

// Conditionally print based on verbose flag
macro_rules! vprintln {
    ($kind:expr, $($arg:tt)*) => {
        if is_verbose() {
            println!("{:>7} {}", $kind.label(), format!($($arg)*));
        }
    };
}

//==============================
//         END MACROS
//==============================


fn main() {
    let args: Vec<String> = env::args().collect();
    let _prog = &args[0]; // Can be removed?

    let mut flags = Options::new();
    set_flags(&mut flags);

    let matches = match flags.parse(&args[1..]) {
        Ok(o) => o,
        Err(e) => {
            // Unconditionally print error message
            println!("[ERROR] {}", e.to_string());
            std::process::exit(1);
        }
    };

    // Process verbose flag first
    let verbose_flag = matches.opt_present("v");
    VERBOSE.set(AtomicBool::new(verbose_flag)).unwrap();
    vprintln!(LogType::INFO, "Verbose flag has been set");
    
    // Temporariy, can be removed
    vprintln!(LogType::INFO, "Testing logging...");
    vprintln!(LogType::INFO, "Test info");
    vprintln!(LogType::WARN, "Test warn");
    vprintln!(LogType::ERROR, "Test error");

    // For testing, can be removed
    if matches.opt_present("test") {
        if let Some(put) = matches.opt_str("test") {
            temp_test(&put);
        } else {
            eprintln!("[ERROR] --test flag requires an argument.");
        }
    }


    // Processing flags passed
    //
    // Surely there's a better way...
    if matches.opt_present("c") {
        bench_cpu();
    }

    if matches.opt_present("d") {
        bench_disk(); 
    }

    if matches.opt_present("f") {
        vprintln!(LogType::INFO, "Running full benchmark...");
    }

    if matches.opt_present("g") {
        bench_gpu();
    }

    if matches.opt_present("m") {
        bench_mem();
    }

    if matches.opt_present("p") {
        bench_processes();
    }

    if matches.opt_present("r") {
        bench_ram();
    }

    if matches.opt_present("s") {
        bench_calls();
    }

    if matches.opt_present("t") {
        bench_time();
    }
}


fn temp_test(arg: &str) {
    vprintln!(LogType::INFO, "Test ran with: {}", arg);
}

fn bench_cpu() {
    vprintln!(LogType::INFO, "Running CPU benchmark");
}


fn bench_disk() {
    vprintln!(LogType::INFO, "Running DISK benchmark");
}


fn bench_gpu() {
    vprintln!(LogType::INFO, "Running GPU benchmark");
}


fn bench_mem() {
    vprintln!(LogType::INFO, "Running memory benchmark");
}


fn bench_processes() {
    vprintln!(LogType::INFO, "Running processes benchmark");
}


fn bench_ram() {
    vprintln!(LogType::INFO, "Running RAM benchmark");
}


fn bench_calls() {
    vprintln!(LogType::INFO, "Running syscalls benchmark");
}


fn bench_time() {
    vprintln!(LogType::INFO, "Running time benchmark");
}


fn set_flags(flags: &mut Options) {
    flags.optflag("c", "cpu", "Benchmarks CPU usage");
    flags.optflag("d", "disk", "Benchmarks disk usage");
    flags.optflag("f", "full", "Benchmark everything");
    flags.optflag("g", "gpu", "Benchmarks GPU usage");
    flags.optflag("m", "memory", "Benchmarks memory");
    flags.optflag("p", "processes", "Benchmark child processes");
    flags.optflag("r", "ram", "Benchmarks RAM usage");
    flags.optflag("s", "sys", "Track all syscalls");
    flags.optflag("t", "time", "Benchmarks time");
    flags.optflag("v", "verbose", "Print debug info");

    // Temp flag for testing
    flags.optopt("", "test", "Temporary test mode", "EXE");
}
