use std::{
    borrow::BorrowMut,
    error::Error,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

use chrono::Utc;
use configurator::configurator::{Config, build_config, parse_args};
use getopts::Matches;

use crate::snap::Snap;

extern crate getopts;

mod configurator;
mod snap;

//
// ./run.sh --exe bin/cpu --full
// intended run usage.  --exe is the test binary, the other flags are what we are tesitng more.
// Just gonna assume all for now
//

fn main() -> Result<(), Box<dyn Error>> {
    let flags: Matches = parse_args()?;

    let conf = build_config(flags).unwrap();

    println!("exe path: {:?}", conf.exe_path);

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
