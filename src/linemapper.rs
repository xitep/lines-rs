use std::io::{Buffer, IoResult, EndOfFile};
use bytes;

/// Maps the given function 'f' over lines read from 'r' until either
/// 'f' returns false or end of file is encountered.
pub fn map_lines<R: Buffer>(mut r: R, f: |&[u8]| -> bool) -> IoResult<()> {
    let mut line_start: Vec<u8> = Vec::new();

    loop {
        let mut consumed;
        {
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
                            b.slice_to(i+1)
                        } else {
                            line_start.push_all(b.slice_to(i+1));
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
        r.consume(consumed);
    }

    if ! line_start.is_empty() {
        if !f(line_start.as_slice()) {
            return Ok(())
        }
    }
    Ok(())
}
