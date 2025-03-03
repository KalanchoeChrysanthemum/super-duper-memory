use std::{thread, time};

pub fn main() {
    println!("Running time benchmark");

    sleep_for_seconds(4);

    // maybe this is unnecessary, but i feel like
    // a benchmark should do more than just sleep for some time
    let mut dummy = vec![0; 64];
    for i in 0..64 {
        dummy[i] = i;
    }

    sleep_for_seconds(3);
    sleep_for_seconds(1);
}

fn sleep_for_seconds(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}
