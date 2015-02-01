#![feature(test, io)]

extern crate lines;
extern crate test;

use std::old_io::BufReader;
use lines::linemapper;
use lines::linereader;

static LINE: &'static str = "foo bar baz quux\n";

fn make_string() -> String {
    let rep = 200us;
    let mut s = String::with_capacity(LINE.len()*rep);
    for _ in 0..rep {
        s.push_str(LINE);
    }
    s
}

#[bench]
fn bench_linemapper_count_lines(b: &mut test::Bencher) {
    let s = make_string();
    b.bytes = s.as_bytes().len() as u64;
    b.iter(|| {
        let r = BufReader::new(s.as_bytes());
        let _ = test::black_box(linemapper::count_lines(r));
    });
}

#[bench]
fn bench_linereader_count_lines(b: &mut test::Bencher) {
    let s = make_string();
    b.bytes = s.as_bytes().len() as u64;
    b.iter(|| {
        let r = linereader::LineReader::new(BufReader::new(s.as_bytes()));
        let _ = test::black_box(linereader::count_lines(r));
    });
}
