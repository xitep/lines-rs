use std::io::{IoResult, EndOfFile, IoError};
use bytes;

pub struct LineReader<'a, R> {
    block: Vec<u8>,

    inner: R,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}

static DEFAULT_BUF_SIZE: usize = 64 * 1024;

impl<'a, R: Reader> LineReader<'a, R> {

    pub fn with_capacity(cap: usize, inner: R) -> LineReader<'a, R> {
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

    pub fn new(inner: R) -> LineReader<'a, R> {
        LineReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    // private
    fn fill_buf(&'a mut self) -> IoResult<()> {
        if self.pos == self.cap {
            self.cap = try!(self.inner.read(&mut self.buf[]));
            self.pos = 0;
        }
        Ok(())
    }

    fn read_until(&'a mut self, byte: u8) -> IoResult<&'a [u8]> {
        // ~ clear our previously delivered block - if any
        unsafe { self.block.set_len(0); }

        loop {
            // ensure we have data to process
            match self.fill_buf() {
                Ok(_) => {},
                Err(e@IoError{kind: EndOfFile, ..}) => {
                    return if self.block.is_empty() {
                        Err(e)
                    } else {
                        Ok(&self.block[])
                    };
                }
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
                        self.block.push_all(&b[..i]);
                        &self.block[]
                    };
                    self.pos += i;
                    return Ok(b);
                }
                None => {
                    self.block.push_all(b);
                    self.pos += b.len();
                    // continue looping
                }
            }
        }
    }

    #[inline]
    pub fn read_line(&'a mut self) -> IoResult<&'a [u8]> {
        self.read_until(b'\n')
    }

}

pub fn count_lines<'a, R: Reader> (mut r: LineReader<'a, R>)
    -> IoResult<usize>
{
    let mut lines = 0us;
    loop {
        match r.read_line() {
            Ok(_) => lines += 1,
            Err(ref e) if e.kind == EndOfFile => break,
            Err(e) => return Err(e),
        }
    }
    Ok(lines)
}
