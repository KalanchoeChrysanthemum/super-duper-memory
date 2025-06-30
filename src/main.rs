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

use charming::component::{Axis, Title};
use charming::datatype::DataPoint;
use charming::element::{AxisType, Tooltip};
use charming::{component::Legend, element::ItemStyle, series::Line, Chart};
use charming::{HtmlRenderer, ImageRenderer};

use colored::{ColoredString, Colorize};
use configurator::configurator::{build_config, parse_args};
use getopts::Matches;

use std::error::Error;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::Duration;

use crate::snap::Snap;

mod configurator;
mod snap;

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

fn main() -> Result<(), Box<dyn Error>> {
    let flags: Matches = parse_args().unwrap();
    let conf = build_config(flags).unwrap();

    // Process verbose flag first
    VERBOSE.set(AtomicBool::new(conf.verbose)).unwrap();
    vprintln!(LogType::INFO, "Verbose flag has been set");

    // Temporariy, can be removed
    vprintln!(LogType::INFO, "Testing logging...");
    vprintln!(LogType::INFO, "Test info");
    vprintln!(LogType::WARN, "Test warn");
    vprintln!(LogType::ERROR, "Test error");

    // Processing flags passed
    //
    // Surely there's a better way...
    if conf.cpu {
        bench_cpu();
    }

    if conf.disk {
        bench_disk();
    }

    if conf.gpu {
        bench_gpu();
    }

    if conf.memory {
        bench_mem();
    }

    if conf.processes {
        bench_processes();
    }

    if conf.ram {
        bench_ram();
    }

    if conf.sys {
        bench_calls();
    }

    if conf.time {
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

    //./run.sh bin/cpu --full intended usage

    println!("{:?}", snaps);

    // let chart = Chart::new()
    //     .x_axis(Axis::new().data(snaps.iter().map(|s| s.time.to_string()).collect()))
    //     .y_axis(Axis::new())
    //     .series(Line::new().data(snaps.iter().map(|s| s.used_memory_in_gb).collect()));
    //

    let chart = Chart::new()
        .title(Title::new().text("Memory Usage Over Time (GB)"))
        .tooltip(Tooltip::new())
        .legend(Legend::new())
        .x_axis(
            Axis::new()
                .name("Time (ms)")
                .type_(AxisType::Category)
                .data(snaps.iter().map(|s| s.time.to_string()).collect()),
        )
        .y_axis(Axis::new().name("Memory (GB)").type_(AxisType::Value))
        .series(
            Line::new()
                .name("Used Memory")
                .smooth(true)
                // .area_style(ItemStyle::new().color("rgba(255, 99, 132, 0.4)"))
                .item_style(ItemStyle::new().color("rgb(255, 99, 132)"))
                .data::<DataPoint>(snaps.iter().map(|s| s.used_memory_in_gb.into()).collect()),
        );
    // let mut renderer = ImageRenderer::new(1000, 800);
    // renderer.save(&chart, "chart.svg")?;
    //

    let html = HtmlRenderer::new("Memory Chart", 800, 1000).render(&chart)?;
    std::fs::write("chart.html", html)?;

    Ok(())
}

// system taken by lib
pub struct UserSystem {
    total_memory_in_gb: f64,
}

// uses system to run proc, but ignore results, updates atomic bool
fn run_exe_in_bg(exe: PathBuf, done_flag: &Arc<AtomicBool>) {
    if let Ok(mut child) = Command::new(exe)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    // uhh new scope
    {
        // I think this clone is fine ;-;
        let done_flag = Arc::clone(done_flag);
        thread::spawn(move || {
            let _ = child.wait();
            done_flag.store(true, Ordering::SeqCst);
        });
    }
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
