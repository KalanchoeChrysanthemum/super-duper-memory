use chrono::Utc;
use sysinfo::System;

// snapshots of system usage at point in time
#[derive(Debug, Clone, Copy)]
pub struct Snap {
    time: chrono::DateTime<Utc>,
    used_memory_in_gb: f64,
}

impl Snap {
    pub fn new(sys: &mut System) -> Self {
        sys.refresh_all();

        let used_memory = sys.used_memory();

        Snap {
            time: Utc::now(),
            used_memory_in_gb: used_memory as f64 / (1024.0 * 1024.0),
        }
    }
}
