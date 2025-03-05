use core::time;
use std::thread;

pub fn main() {
    println!("Running memory benchmark...");
    let mut fill_me: Vec<i128> = vec![];
    // should be 8 gigs if im understanding this

    let size: i128 = 1024 * 1024 * 256;

    // filll with 2 gigs
    for num in 1..size {
        fill_me.push(num);
    }

    // sleep for 3 secs
    // wanna be able to see how memory changes over time
    thread::sleep(time::Duration::from_secs(3));

    // put another 5 gigs in it
    for num in 1..size {
        fill_me.push(num);
    }
    // most of this should end up in swap space, which should be interesting

    println!("Finished memory benchmark");
}

