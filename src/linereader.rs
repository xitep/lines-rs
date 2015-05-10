//! Iterates over lines of a given reader by keeping an internal
//! buffer which client code is handed a reference to for consumption
//! of the line.
//!
//! Since this module implements internal buffering, there is no need
//! for clients to supply a buffering reader implementation.

use std::io::{Read, Result};
use bytes;

/// Wraps a `Read` and provides a way to iterate over lines split on a
/// newline character.
///
/// Unlike `BufReaderExt::lines` a line provided by
/// `LineReader::read_line` is represented as a slice into an internal
/// buffer which gets overwritten upon the next invocation of this
/// method.  This allows clients to read through lines without
/// enforcing memory allocation on every line.  On the other hand,
/// clients must copy the provided data if a line is to be kept for
/// later reference.
///
/// # Examples
///
/// Iterating over all the lines of a given `Read` can be implemented
/// as follows:
///
/// ```text
/// extern crate lines;
///
/// let r: Read = ...;
/// let mut lr = LineReader::new(r);
/// loop {
///   match lr.read_line() {
///      Ok(b) if b.is_empty() => { /* end of file */ }
///      Ok(line) => { /* process line bytes */ }
///      Err(e) => { /* i/o error occured */ }
///   }
/// }
/// ```
///
/// For convenience, you'd usually use the `read_lines` macro provided
/// by this module.  The macro will loop over the lines and take care
/// of aborting the loop when encountering end-of-file.
///
/// ```text
/// #[macro_use(read_lines)]
/// extern crate lines;
///
/// let r: Read = ...;
/// let mut reader = LineReader::new(r);
/// read_lines(line in reader, {
///   match line {
///     Ok(line) => { /* process line bytes; line is never empty */ }
///     Err(e)   => { /* i/o error occured */ }
///   }
/// });
/// ```
pub struct LineReader<R> {
    block: Vec<u8>,

    inner: R,
    buf: Vec<u8>,
    pos: usize,
    cap: usize,
}

static DEFAULT_BUF_SIZE: usize = 64 * 1024;

impl<R: Read> LineReader<R> {

    /// Constructs a new `LineReader` with an internal buffering of
    /// the specified capacity.
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

    /// Constructs a new `LineReader` with an internal buffering of
    /// the default capacity.
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

    /// Reads the next line. Returns the empty slice if EOF is
    /// reached.  The newline character - if any - is not stripped by
    /// this method.
    #[inline]
    pub fn read_line(&mut self) -> Result<&[u8]> {
        self.read_until(b'\n')
    }
}

/// Provides a convenient way to iterate over all lines through a
/// `LineReader`.  For an example, refer to the documentation of
/// `LineReader`.  The identifier representing the read line will be
/// of type `Result<&[u8]>`.
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

/// Provides a convenient way to iterate over all lines through a
/// `LineReader` wrapping the invocation to `LineReader::read_line`
/// with a `try!`. Hence, the identifer representing the read line
/// will be directly of type `&[u8]`.
#[macro_export]
macro_rules! try_read_lines {
    ($inp:ident in $expr:expr, $b:block) => {
        { let ref mut r = &mut $expr;
          loop {
              let $inp = try!(r.read_line());
              if $inp.is_empty() { break; }
              else { $b };
          };
        }
    }
}

/// Counts the lines read through the given `LineReader`.
pub fn count_lines<R: Read> (mut r: LineReader<R>)
    -> Result<usize>
{
    let mut lines = 0usize;
    try_read_lines!(line in r, {
        lines += 1;
    });
    Ok(lines)
}
