extern crate clap;
extern crate libc;

use std::process::exit;
use std::fs::OpenOptions;
use std::io::{self, stdin, stdout, stderr, Read, Write};
use std::path::Path;
use clap::{App, Arg};

fn prepend(stdin_buffer: &[u8], file: &Path) -> io::Result<()> {
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

fn print_to_stdout(file: &Path) -> io::Result<()> {
    let mut openfile = try!{
        OpenOptions::new()
            .read(true)
            .open(file)
    };

    let mut file_buffer: Vec<u8> = Vec::new();
    try!(openfile.read_to_end(&mut file_buffer));
    try!(stdout().write_all(&file_buffer));

    Ok(())
}

fn can_write(file: &Path) -> bool {
    OpenOptions::new()
            .write(true)
            .create(true)
            .open(file)
            .map_err(|e| print_error(format!("Writing to file {} failed: {}\n", file.to_string_lossy(), e)))
            .is_ok()
}

fn print_error(error: String) {
    let _ = stderr().write_all(error.as_bytes());
}

fn main() {
    let mut exitcode = 0;

    let matches = App::new("prepend")
        .version("2.2.1")
        .about("Prepends data to a file")
        .arg(Arg::with_name("tee")
             .short("t")
             .long("tee")
             .help("Print new file contents to stdout"))
        .arg(Arg::with_name("FILE")
             .index(1)
             .required(true)
             .multiple(true)
             .help("File(s) to prepend data to"))
        .get_matches();

    unsafe {
        if libc::fcntl(0, libc::F_GETFD) != 0 {
            print_error(format!("Failed to read from stdin\n"));
            exit(3);
        }
    }

    let mut files_vec = Vec::new();
    for file in matches.values_of_os("FILE").unwrap() {
        let file = Path::new(file);
        if can_write(file) {
            files_vec.push(file);
        } else {
            exitcode = 4;
        }
    }

    if files_vec.is_empty() {
        exit(2);
    }

    let mut stdin_buffer = Vec::new();
    if let Err(e) = stdin().read_to_end(&mut stdin_buffer) {
        print_error(format!("Failed to read from stdin: {}\n", e));
        exit(3);
    }

    for file in files_vec {
        match prepend(&stdin_buffer, &file) {
            Ok(_) => {
                if matches.is_present("tee") {
                    if let Err(e) = print_to_stdout(&file) {
                        print_error(format!("Printing to stdout failed for file {}: {}\n", file.display(), e));
                        exitcode = 4;
                    }
                }
            },
            Err(e) => {
                print_error(format!("Writing to file {} failed: {}\n", file.display(), e));
                exitcode = 4;
            },
        }
    }

    exit(exitcode);
}
