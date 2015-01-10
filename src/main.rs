extern crate lines;

use std::os;
use std::io;
use lines::linemapper;

fn main() {
    for arg in os::args().iter().skip(1) {
        match process_arg(arg) {
            Ok(lines) => println!("{}: {}", arg, lines),
            Err(e) => println!("{}: {}", arg, e),
        }
    }
}

fn process_arg(arg: &String) -> io::IoResult<usize> {
    let f = try!(io::File::open(&Path::new(arg)));
    linemapper::count_lines(io::BufferedReader::new(f))
}
