// Declares external libraries
extern crate clap;
extern crate libc;

// Imports things into namespace
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

    // Read the contents of the given file into a buffer
    let mut file_buffer: Vec<u8> = Vec::new();
    try!(openfile.read_to_end(&mut file_buffer));

    // Truncate the file to zero length (i.e. delete the contents of the file)
    try!(openfile.set_len(0));
    // Write the contents of the standrad input buffer to the file
    try!(openfile.write_all(&stdin_buffer));
    // Write the contents of the file buffer (i.e. the original contents of the file)
    // to the file
    try!(openfile.write_all(&file_buffer));

    Ok(())
}

// Reads the given file and prints its contents to standard output
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

// Tries to open the given file in write mode
// Returns true if the file can be written to
// and false if it can't
fn can_write(file: &Path) -> bool {
    OpenOptions::new()
            .write(true)
            .create(true)
            .open(file)
            // Passes through successful Result, or handles the error
            .map_err(|e| print_error(format!("Writing to file {} failed: {}\n", file.to_string_lossy(), e)))
            .is_ok()
}

// Writes the given string to standard error
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
        // Checks to see if standard input is closed
        // e.g. ''prepend file 0<&-''
        // Exits with exit value of 3 if it isn't
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

    // If no given files can be written to,
    // exit with exit value of 2
    if files_vec.is_empty() {
        exit(2);
    }

    // Create a new Vector, then fill it with the contents of
    // standard input
    let mut stdin_buffer = Vec::new();
    // Check to see if read_to_end returns an Err
    if let Err(e) = stdin().read_to_end(&mut stdin_buffer) {
        print_error(format!("Failed to read from stdin: {}\n", e));
        exit(3);
    }

    // Iterate over the files in files_vec
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
