use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::exit;
use std::{env, ffi};

fn prepend(stdin_buffer: &Vec<u8>, file: &ffi::OsString) -> io::Result<()> {
    let mut openfile = try!{
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file)
    };

    let mut file_buffer: Vec<u8> = Vec::new();
    try!(openfile.read_to_end(&mut file_buffer));

    try!(openfile.set_len(0));
    try!(openfile.write_all(&stdin_buffer));
    try!(openfile.write_all(&file_buffer));

    Ok(())
}

fn main() {
    if env::args_os().count() < 2 {
        println!("No file(s) specified");
        exit(1);
    }

    let mut stdin_buffer: Vec<u8> = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut stdin_buffer) {
        println!("Failed to read from stdin: {}", e);
        exit(1);
    }

    for file in env::args_os().skip(1) {
        if let Err(e) = prepend(&stdin_buffer, &file) {
            println!("Writing to file {} failed: {}", file.to_string_lossy(), e);
        }
    }
}
