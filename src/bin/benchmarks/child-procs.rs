use std::process::{Command, Child};
use std::{thread, time};

use rand::Rng;

fn spawn_child() -> Child {
    let child = Command::new("sleep")
        .arg("10") 
        .spawn()
        .expect("Failed to start child process");

    let num = rand::rng().random_range(0..10);

    if num == 0 {
        spawn_child();
    }


    child
    
}

fn main() {
    // i had to use `ps --forest -e` to get most of these to show up
    // should in theory work for children of children

    let num_procs = 10; 

    let mut children_procs = Vec::new();

    for _ in 0..num_procs {
        children_procs.push(spawn_child());
    }

    for mut child in children_procs {
        let _ = child.wait().expect("Failed to wait on child");
    }
}
