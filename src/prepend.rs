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

fn can_write(file: &ffi::OsString) -> bool {
    if let Err(e) = OpenOptions::new()
            .write(true)
            .create(true)
            .open(file) {
                    print_error(format!("Writing to file {} failed: {}\n", file.to_string_lossy(), e));
                    false
            } else {
                true
            }
}

fn print_error(error: String) {
    let _ = io::stderr().write_all(error.as_bytes());
}

fn main() {
    if env::args_os().count() < 2 {
        print_error(format!("No file(s) specified\n"));
        exit(1);
    }

    let mut files_vec: Vec<ffi::OsString> = Vec::new();

    for file in env::args_os().skip(1) {
        if can_write(&file) {
            files_vec.push(file);
        }
    }

    if files_vec.len() == 0 {
        exit(1);
    }

    let mut stdin_buffer: Vec<u8> = Vec::new();
    if let Err(e) = io::stdin().read_to_end(&mut stdin_buffer) {
        print_error(format!("Failed to read from stdin: {}\n", e));
        exit(1);
    }

    for file in files_vec {
        if let Err(e) = prepend(&stdin_buffer, &file) {
            print_error(format!("Writing to file {} failed: {}\n", file.to_string_lossy(), e));
        }
    }
}
