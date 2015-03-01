use std::io::{BufRead, Result};
use bytes;

pub fn count_lines<R: BufRead>(r: R) -> Result<usize> {
    let mut lines = 0usize;
    try!(map_lines(r, |_| { lines += 1; true }));
    Ok(lines)
}

/// Maps the given function 'f' over lines read from 'r' until either
/// 'f' returns false or end of file is encountered.
pub fn map_lines<R, F>(mut r: R, mut f: F) -> Result<()> 
    where R: BufRead, F: FnMut(&[u8]) -> bool
{
    let mut line_start: Vec<u8> = Vec::new();

    let mut consumed = 0usize;
    loop {
        r.consume(consumed);

        let b = match r.fill_buf() {
            Ok(b) => {
                if b.len() == 0 {
                    break;
                }
                b
            },
            Err(e) => { return Err(e); }
        };

        match bytes::index(b, b'\n') {
            Some(i) => {
                {
                    let b = if line_start.is_empty() {
                        &b[..i+1]
                    } else {
                        line_start.push_all(&b[..i+1]);
                        &line_start[..]
                    };
                    if !f(b) {
                        return Ok(())
                    }
                }
                if ! line_start.is_empty() {
                    unsafe { line_start.set_len(0); }
                }
                consumed = i+1;
            }
            None => {
                line_start.push_all(b);
                consumed = b.len();
            }
        }
    }

    if ! line_start.is_empty() {
        f(&line_start[..]);
    }
    Ok(())
}
