use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

fn main() {

    // run with `strace -o tmp.txt ./bin/sys-calls` to view actual syscalls

    let path = "testfile.txt";

    // many threads
    // gives me many concur operatiosn

    let join_handlers: Vec<_> = (0..10)
        .map(|i| {
            // should be a bunch of clone()'s
            thread::spawn(move || {
                for _ in 0..100 {
                    // OpenOption does cool things
                    // some of those opts should match to more system operations
                    let mut file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(path)
                        .expect("Failed to open file");

                    let mut buffer = String::new();
                    file.read_to_string(&mut buffer).expect("couldnt read");

                    // should do like a ton of mmap() calls concurently
                    file.write_all(format!("thread number {} writing\n", i).as_bytes())
                        .expect("Failed to write");
                }
            })
        })
        .collect();


    
    for handle in join_handlers {
        handle.join().unwrap();
    }

    cleanup(path);
}


fn cleanup(path: &str){
    fs::remove_file(path).expect("could not delete dummy file");
}
