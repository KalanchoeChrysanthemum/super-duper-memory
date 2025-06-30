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

use colored::{ColoredString, Colorize};
use configurator::configurator::{build_config, parse_args};
use getopts::Matches;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;

mod configurator;

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
    let flags: Matches = parse_args().unwrap();
    let config = build_config(flags).unwrap();

    // Process verbose flag first
    VERBOSE.set(AtomicBool::new(config.verbose)).unwrap();
    vprintln!(LogType::INFO, "Verbose flag has been set");

    // Temporariy, can be removed
    vprintln!(LogType::INFO, "Testing logging...");
    vprintln!(LogType::INFO, "Test info");
    vprintln!(LogType::WARN, "Test warn");
    vprintln!(LogType::ERROR, "Test error");

    // Processing flags passed
    //
    // Surely there's a better way...
    if config.cpu {
        bench_cpu();
    }

    if config.disk {
        bench_disk();
    }

    if config.gpu {
        bench_gpu();
    }

    if config.memory {
        bench_mem();
    }

    if config.processes {
        bench_processes();
    }

    if config.ram {
        bench_ram();
    }

    if config.sys {
        bench_calls();
    }

    if config.time {
        bench_time();
    }

    let mut sys = sysinfo::System::new_all();

    sys.refresh_memory();

    let user_sys = UserSystem {
        total_memory_in_gb: sys.total_memory() as f64 / (1024.0 * 1024.0),
    };

    let mut snaps: Vec<Snap> = vec![];
    let exe_is_done = Arc::new(AtomicBool::new(false));

    run_exe_in_bg(conf.exe_path, &exe_is_done);

    let time_between_snaps = Duration::from_millis(300);

    while !exe_is_done.load(std::sync::atomic::Ordering::SeqCst) {
        snaps.push(Snap::new(&mut sys));
        thread::sleep(time_between_snaps);
    }

    println!("{:?}", snaps);

    Ok(())
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
