
pub mod configurator {
    
    use getopts::{Matches, Options};
    use std::{env, error::Error};
    
    // eventually wanna be able to serialize this
    // right now these are just bools for "should_run", but can evntually be xpanded into full config types

    #[derive(Debug)]
    pub struct Config {
        time: bool,
        memory: bool,
        disk: bool,
        ram: bool,
        cpu: bool,
        gpu: bool,
        processes: bool,
        sys: bool,
    }
    
    
    
    pub fn build_config(flags: Matches) -> Config {
        // will need to modify this logic once config gets more complex
        Config {
            time: flags.opt_present("time"),
            memory: flags.opt_present("memory"),
            disk: flags.opt_present("disk"),
            ram: flags.opt_present("ram"),
            cpu: flags.opt_present("cpu"),
            gpu: flags.opt_present("gpu"),
            processes: flags.opt_present("processes"),
            sys: flags.opt_present("sys"),
        }
    }
    
    
    pub fn parse_args() -> Result<getopts::Matches ,Box<dyn Error>> {
        let args: Vec<String> = env::args().collect();
        let ref _prog = args[0];
        
        let mut flags = Options::new();
        set_flags(&mut flags);

        let opts = flags.parse(&args[1..])?;
        
        Ok(opts)
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
    
    
}