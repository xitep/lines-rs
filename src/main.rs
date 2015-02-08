#![feature(io, path, core, env, os)]

extern crate lines;

use std::env;
use std::old_io::{BufferedReader, File, IoResult};
use lines::linemapper;

fn main() {
    for arg in env::args().skip(1) {
        let arg = arg.into_string().unwrap();
        match process_arg(&arg) {
            Ok(lines) => println!("{}: {}", arg, lines),
            Err(e) => println!("{}: {}", arg, e),
        }
    }
}

fn process_arg(arg: &String) -> IoResult<usize> {
    let f = try!(File::open(&Path::new(arg)));
    linemapper::count_lines(BufferedReader::new(f))
}
