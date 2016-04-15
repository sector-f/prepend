use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::exit;
use std::{env, ffi};

fn prepend(stdin_buffer: &String, file: &ffi::OsString) -> io::Result<()> {
    let mut openfile = try!{
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file)
    };

    let mut file_buffer = String::new();
    try!(openfile.read_to_string(&mut file_buffer));

    try!(openfile.set_len(0));
    try!(openfile.write(stdin_buffer.as_bytes()));
    try!(openfile.write(file_buffer.as_bytes()));

    Ok(())
}

fn main() {
    if env::args_os().count() < 2 {
        println!("No file(s) specified");
        exit(1);
    }

    let mut stdin_buffer = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut stdin_buffer) {
        println!("Failed to read from stdin: {}", e);
        exit(1);
    }

    for file in env::args_os().skip(1) {
        if let Err(e) = prepend(&stdin_buffer, &file) {
            println!("Writing to file {} failed: {}", file.to_string_lossy(), e);
        }
    }
}
