#![feature(test)]

extern crate lines;
extern crate test;

use std::io::{Result, Read, BufRead, BufReader};
use lines::linemapper;
use lines::linereader;

static LINE: &'static str = "foo bar baz quux\n";

fn make_string() -> String {
    let rep = 200usize;
    let mut s = String::with_capacity(LINE.len()*rep);
    for _ in 0..rep {
        s.push_str(LINE);
    }
    s
}

#[bench]
fn bench_linemapper_count_lines(b: &mut test::Bencher) {
    let s = make_string();
    let data = s.as_bytes();
    b.bytes = data.len() as u64;

    b.iter(|| {
        let r = BufReader::new(data);
        let _ = test::black_box(linemapper::count_lines(r));
    });
}

#[bench]
fn bench_linereader_count_lines(b: &mut test::Bencher) {
    let s = make_string();
    let data = s.as_bytes();
    b.bytes = data.len() as u64;

    b.iter(|| {
        let r = linereader::LineReader::new(data);
        let _ = test::black_box(linereader::count_lines(r));
    });
}

#[bench]
fn bench_bufreader_read_line(b: &mut test::Bencher) {
    let s = make_string();
    let data = s.as_bytes();
    b.bytes = data.len() as u64;

    b.iter(|| {
        let _ = test::black_box(count_lines_using_bufreader_read_line(data));
    });
}

/// Count the lines by reading lines using the std lib's
/// BufReader#read_line method. Note: that this method does UTF-8
/// validation (whereas linemapper and linereader do not.)
fn count_lines_using_bufreader_read_line<R: Read>(r: R) -> Result<usize> {
    let mut lines = 0usize;
    let mut br = BufReader::new(r);
    let mut line = String::with_capacity(512);
    loop {
        try!(br.read_line(&mut line));
        if line.is_empty() {
            break;
        }
        line.clear();
        lines += 1;
    }
    Ok(lines)
}
