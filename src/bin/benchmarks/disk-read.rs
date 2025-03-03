use std::{
    fs,
    io::{BufReader, Read},
};

pub fn main() {
    println!("Running disk-read benchmark");

    // im just using dev random, because I didn't wanna have to write to a new file first
    // (also tossed around just using the side effect from disk-write)
    let path = "/dev/random";

    let file = fs::File::open(path).expect("Couldn't open the /dev/random");

    let mut reader = BufReader::new(file);
    // read 3 gigs from fs into memory
    let mut buffer = vec![0; 1024 * 1024 * 1024 * 3];

    reader
        .read_exact(&mut buffer)
        .expect("Could not read bytes");
    println!("Finished disk-read benchmark");
}
