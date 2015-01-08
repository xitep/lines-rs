use std::io::{Buffer, IoResult, EndOfFile};
use bytes;

pub fn count_lines<R: Buffer>(r: R) -> IoResult<uint> {
    let mut lines = 0u;
    try!(map_lines(r, |_| { lines += 1; true }));
    Ok(lines)
}

/// Maps the given function 'f' over lines read from 'r' until either
/// 'f' returns false or end of file is encountered.
pub fn map_lines<R, F>(mut r: R, mut f: F) -> IoResult<()> 
    where R: Buffer, F: FnMut(&[u8]) -> bool
{
    let mut line_start: Vec<u8> = Vec::new();

    let mut consumed = 0u;
    loop {
        r.consume(consumed);

        let b = match r.fill_buf() {
            Ok(b)  => b,
            Err(ref e) if e.kind == EndOfFile => {
                break;
            }
            Err(e) => {
                return Err(e);
            }
        };

        match bytes::index(b, b'\n') {
            Some(i) => {
                {
                    let b = if line_start.is_empty() {
                        b[..i+1]
                    } else {
                        line_start.push_all(b[..i+1]);
                        line_start.as_slice()
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
        f(line_start.as_slice());
    }
    Ok(())
}
