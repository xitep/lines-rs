#[macro_use(read_lines)]
extern crate lines;

use std::io::{BufReader};
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
        lines.push(String::from_utf8_lossy(line).into_owned());
        true
    }).unwrap();
    let expected: Vec<&str> = vec!["one\n", "two\n", "three\n", "four\n", "five\n", "six\n", "seven\n", "eight\n", "nine\n", "ten"];
    let actual: Vec<&str> = lines.iter().map(|s| &s[..]).collect();
    assert_eq!(expected, actual);
}

#[test]
fn test_linereader_lines() {
    let expected = ["one\n", "two\n", "three\n", "four\n", "five\n", "six\n", "seven\n", "eight\n", "nine\n", "ten"];
    let mut r = linereader::LineReader::new(BufReader::new(TEN_LINES.as_bytes()));
    let mut i = 0usize;
    read_lines!(line in r, {
        let line = line.unwrap();
        assert_eq!(expected[i], &String::from_utf8_lossy(line)[..]);
        i += 1;
    });
}

#[test]
fn test_linemapper_linecount() {
    let r = BufReader::new(TEN_LINES.as_bytes());
    assert_eq!(10usize, linemapper::count_lines(r).unwrap());
}

#[test]
fn test_linereader_linecount() {
    let r = BufReader::new(TEN_LINES.as_bytes());
    // ~ use very small capacity to trigger the overflow 
    // logic inside LineReader
    let r = linereader::LineReader::with_capacity(4, r);
    assert_eq!(10usize, linereader::count_lines(r).unwrap());
}
