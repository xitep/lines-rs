#![feature(io, os, path)]

extern crate lines;

use std::os;
use std::old_io::{BufferedReader, File, IoResult};
use lines::linemapper;

fn main() {
    for arg in os::args().iter().skip(1) {
        match process_arg(arg) {
            Ok(lines) => println!("{}: {}", arg, lines),
            Err(e) => println!("{}: {}", arg, e),
        }
    }
}

fn process_arg(arg: &String) -> IoResult<usize> {
    let f = try!(File::open(&Path::new(arg)));
    linemapper::count_lines(BufferedReader::new(f))
}
