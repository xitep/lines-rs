use std::io::{Read, Result};
use bytes;

pub struct LineReader<R> {
    block: Vec<u8>,

    inner: R,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}

static DEFAULT_BUF_SIZE: usize = 64 * 1024;

impl<R: Read> LineReader<R> {

    pub fn with_capacity(cap: usize, inner: R) -> LineReader<R> {
        let mut buf = Vec::with_capacity(cap);
        unsafe { buf.set_len(cap); }
        LineReader {
            block: Vec::new(),

            inner: inner,
            buf: buf,
            pos: 0,
            cap: 0,
        }
    }

    pub fn new(inner: R) -> LineReader<R> {
        LineReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    // private
    fn fill_buf<'a>(&'a mut self) -> Result<usize> {
        if self.pos == self.cap {
            self.cap = try!(self.inner.read(&mut self.buf[..]));
            self.pos = 0;
        }
        Ok(self.cap-self.pos)
    }

    fn read_until<'a>(&'a mut self, byte: u8) -> Result<&'a [u8]> {
        // ~ clear our previously delivered block - if any
        unsafe { self.block.set_len(0); }

        loop {
            // ensure we have data to process
            match self.fill_buf() {
                Ok(0) => {
                    return Ok(&self.block[..]);
                },
                Ok(_) => {},
                Err(e) => return Err(e),
            };

            // note: we're dealing here with buf, pos, cap directly
            // to avoid the mutability checker from getting in our way
            let b = &self.buf[self.pos .. self.cap];
            match bytes::index(b, byte) {
                Some(mut i) => {
                    i += 1;
                    let b = if self.block.is_empty() {
                        &self.buf[self.pos .. self.pos+i]
                    } else {
                        let b: &[u8] = &b[..i];
                        self.block.extend(b.iter().cloned());
                        &self.block[..]
                    };
                    self.pos += i;
                    return Ok(b);
                }
                None => {
                    self.block.extend(b.iter().cloned());
                    self.pos += b.len();
                    // continue looping
                }
            }
        }
    }

    /// Reads the next line. Returns the empty slice if EOF is reached
    /// and there's not more data to deliver.
    #[inline]
    pub fn read_line(&mut self) -> Result<&[u8]> {
        self.read_until(b'\n')
    }

}

#[macro_export]
macro_rules! read_lines {
    ($inp:ident in $expr:expr, $b:block) => {
        { let ref mut r = &mut $expr;
          loop {
              let $inp = r.read_line();
              match $inp {
                  Ok(b) if b.is_empty() => {
                      break
                  }
                  _ => { $b }
              };
          };
        }
    }
}

pub fn count_lines<R: Read> (mut r: LineReader<R>)
    -> Result<usize>
{
    let mut lines = 0usize;
    read_lines!(line in r, {
        match line {
            Err(e) => return Err(e),
            Ok(_) => lines += 1,
        }
    });
    Ok(lines)
}
