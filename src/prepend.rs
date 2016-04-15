use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::exit;
use std::env;

fn main() {
    let file = match env::args_os().nth(1) {
        Some(f) => f,
        None => {
            println!("No file specified");
            exit(1);
        },
    };

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

    let mut buffer = String::new();
    let _ = openfile.read_to_string(&mut buffer);

    let mut stdin = String::new();
    let _ = io::stdin().read_to_string(&mut stdin);

    let _ = openfile.set_len(0);
    let _ = openfile.write(stdin.as_bytes());
    let _ = openfile.write(buffer.as_bytes());
}
