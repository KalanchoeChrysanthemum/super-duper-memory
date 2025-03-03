use rand::Rng;
use std::{
    fs::{create_dir_all, remove_dir_all, File}, io::Write
};

pub fn main() {
    println!("Running disk benchmark");

    if create_dir_all("tmp").is_err() {
        println!("oopsies could not create tmp dir");
        cleanup();
    }

    let filename = "dummy_data.txt";
    let path = format!("tmp/{}", filename);

    let mut f = match File::create(path) {
        Ok(f) => f,
        Err(_) => {
            println!("oopsies could not create file");
            cleanup();
            return ();// gotta be a better way to handle this buttttt
        }
    };

    let num_chars_to_write = 3_000_000;

    for _ in 0..num_chars_to_write {
        let num = rand::rng().random_range(32..126);
        let ascii_char = num as u8;

        // need u8 as slice bc f.write() doesnt support a single char
        let writeable_char: &[u8] = std::slice::from_ref(&ascii_char);
        if f.write(writeable_char).is_err() {
            cleanup();
        }
    }

    cleanup();
}

fn cleanup() {
    if remove_dir_all("tmp").is_err() {
        println!("oopsies could not clean tmp dir");
    }

    println!("Finished memory benchmark");

    std::process::exit(0);
}
