pub mod configurator {

    use getopts::{Matches, Options};
    use std::{
        env,
        error::Error,
        fs,
        path::{self, PathBuf},
    };

    // eventually wanna be able to serialize this
    // right now these are just bools for "should_run", but can evntually be xpanded into full config types

    #[derive(Debug)]
    pub struct Config {
        pub time: bool,
        pub memory: bool,
        pub disk: bool,
        pub ram: bool,
        pub cpu: bool,
        pub gpu: bool,
        pub processes: bool,
        pub sys: bool,
        pub exe_path: PathBuf,
    }

    fn path_resolve(path: &str) -> Result<PathBuf, Box<dyn Error>> {
        let src = PathBuf::from(path);

        Ok(fs::canonicalize(src)?)
    }

    pub fn build_config(flags: Matches) -> Result<Config, Box<dyn Error>> {
        let exe_path: String = flags
            .opt_get("exe")?
            .ok_or_else(|| String::from("Could not get executable path"))?;
        let full_exe_path = path_resolve(&exe_path)?;
        let full = flags.opt_present("full");

        // will need to modify this logic once config gets more complex
        Ok(Config {
            time: full || flags.opt_present("time"),
            memory: full || flags.opt_present("memory"),
            disk: full || flags.opt_present("disk"),
            ram: full || flags.opt_present("ram"),
            cpu: full || flags.opt_present("cpu"),
            gpu: full || flags.opt_present("gpu"),
            processes: full || flags.opt_present("processes"),
            sys: full || flags.opt_present("sys"),
            exe_path: full_exe_path,
        })
    }

    pub fn parse_args() -> Result<getopts::Matches, Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
        let ref _prog = args[0];

        let mut flags = Options::new();
        set_flags(&mut flags);

        let opts = flags.parse(&args[1..])?;

        Ok(opts)
    }

    fn set_flags(flags: &mut Options) {
        flags.optflag("t", "time", "Benchmarks time");
        flags.optflag("m", "memory", "Benchmarks memory");
        flags.optflag("d", "disk", "Benchmarks disk usage");
        flags.optflag("r", "ram", "Benchmarks RAM usage");
        flags.optflag("c", "cpu", "Benchmarks CPU usage");
        flags.optflag("g", "gpu", "Benchmarks GPU usage");
        flags.optflag("p", "processes", "something children processes?");
        flags.optflag("s", "sys", "Track all syscalls");
        flags.optflag("f", "full", "Benchmark everything");

        // always need executable path
        flags.reqopt("e", "exe", "Path to executable", "EXE_FILE");
    }
}
