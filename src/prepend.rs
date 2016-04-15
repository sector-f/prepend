use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::exit;
use std::{env, ffi};

fn prepend(stdin_buffer: &String, file: ffi::OsString) {
    let mut openfile = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file) {
            Ok(f) => f,
            Err(e) => {
                println!("Error: {}", e);
                exit(1);
            },
        };

    let mut file_buffer = String::new();
    let _ = openfile.read_to_string(&mut file_buffer);

    let _ = openfile.set_len(0);
    let _ = openfile.write(stdin_buffer.as_bytes());
    let _ = openfile.write(file_buffer.as_bytes());
}

fn main() {
    if env::args_os().count() < 2 {
        println!("No file(s) specified");
        exit(1);
    }

    let mut stdin_buffer = String::new();
    let _ = io::stdin().read_to_string(&mut stdin_buffer);

    for file in env::args_os().skip(1) {
        prepend(&stdin_buffer, file);
    }
}
