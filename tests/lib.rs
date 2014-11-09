extern crate lines;

use std::io::{BufReader, EndOfFile};
use lines::linemapper;
use lines::linereader;

static TEN_LINES: &'static str = "one
two
three
four
five
six
seven
eight
nine
ten";

#[test]
fn test_linemapper_lines() {
    let r = BufReader::new(TEN_LINES.as_bytes());
    let mut lines = Vec::<String>::new();
    linemapper::map_lines(r, |line| {
        let s = match String::from_utf8_lossy(line) {
            std::str::Slice(slc) => String::from_str(slc),
            std::str::Owned(s) => s,
        };
        lines.push(s);
        true
    }).unwrap();
    assert_eq!(vec!("one\n", "two\n", "three\n", "four\n", "five\n", "six\n", "seven\n", "eight\n", "nine\n", "ten"),
               lines.iter().map(|s| s.as_slice()).collect());
}

#[test]
fn test_linereader_lines() {
    let expected = ["one\n", "two\n", "three\n", "four\n", "five\n", "six\n", "seven\n", "eight\n", "nine\n", "ten"];
    let mut r = linereader::LineReader::new(BufReader::new(TEN_LINES.as_bytes()));
    let mut i = 0u;
    loop {
        match r.read_line() {
            Ok(line) => {
                assert_eq!(expected[i], String::from_utf8_lossy(line).as_slice());
            }
            Err(ref e) if e.kind == EndOfFile => break,
            Err(e) => panic!(e),
        }
        i += 1;
    }
}

#[test]
fn test_linemapper_linecount() {
    let r = BufReader::new(TEN_LINES.as_bytes());
    assert_eq!(Ok(10u), linemapper::count_lines(r));
}

#[test]
fn test_linereader_linecount() {
    let r = BufReader::new(TEN_LINES.as_bytes());
    // ~ use very small capacity to trigger the overflow 
    // logic inside LineReader
    let r = linereader::LineReader::with_capacity(4, r);
    assert_eq!(Ok(10u), linereader::count_lines(r));
}
