#![allow(dead_code)]

use std::io::{IoResult, EndOfFile, IoError};
use bytes;

pub struct LineReader<'a, R> {
    inner: R,
    buf: Vec<u8>,
    pos: uint,
    cap: uint,

    consumed: uint,
    block: Vec<u8>,
}

static DEFAULT_BUF_SIZE: uint = 64 * 1024;

impl<'a, R: Reader> LineReader<'a, R> {

    pub fn with_capacity<'a>(cap: uint, inner: R) -> LineReader<'a, R> {
        let mut buf = Vec::with_capacity(cap);
        unsafe { buf.set_len(cap); }
        LineReader {
            inner: inner,
            buf: buf,
            pos: 0,
            cap: 0,

            consumed: 0,
            block: Vec::new(),
        }
    }

    pub fn new<'a>(inner: R) -> LineReader<'a, R> {
        LineReader::with_capacity(DEFAULT_BUF_SIZE, inner)
    }

    // private
    fn fill_buf(&'a mut self) -> IoResult<uint> {
        if self.pos == self.cap {
            self.cap = try!(self.inner.read(self.buf[mut]));
            self.pos = 0;
        }
        Ok(self.cap-self.pos)
    }

    fn read_until(&'a mut self, byte: u8) -> IoResult<&'a [u8]> {
        // ~ clear our previously delivered block - if any
        unsafe { self.block.set_len(0); }

        loop {

            // is there anything we have consumed the last time
            // read_until was called? if so, do actually consume
            // it now; we do this here in place to work around
            // the mutability checker
            self.pos += self.consumed;
            self.consumed = 0;

            // ensure we have data to process
            let avail = match self.fill_buf() {
                Ok(n) => n,
                Err(IoError{kind: EndOfFile, ..}) if !self.block.is_empty() => 0,
                Err(e) => return Err(e),
            };

            if avail == 0 {
                return Ok(self.block.as_slice());
            }

            // note: we're dealing here with buf, pos, cap directly
            // to avoid the mutability checker from getting in our way
            let b = self.buf.slice(self.pos,self.cap);
            match bytes::index(b, byte) {
                Some(i) => {
                    if !self.block.is_empty() {
                        self.block.push_all(b.slice_to(i+1));
                    }
                    self.consumed = i+1;
                    // ideally would return directly from here either
                    // referencing 'b' or 'self.block' but the borrow
                    // checker'd be stepping in :/ ... so instead, we
                    // break out of the loop and evaluate what to 
                    // return based on the emptiness of 'self.block'
                    break;
                }
                None => {
                    self.block.push_all(b);
                    self.consumed = b.len();
                    // continue looping
                }
            }

        }

        if self.block.is_empty() {
            Ok(self.buf.slice(self.pos, (self.pos+self.consumed)))
        } else {
            Ok(self.block.as_slice())
        }
    }

    pub fn read_line(&'a mut self) -> IoResult<&'a [u8]> {
        self.read_until(b'\n')
    }

}
