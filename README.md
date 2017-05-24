lines - streaming through text line by line
===========================================

[![Build Status](https://travis-ci.org/xitep/lines-rs.svg?branch=master)](https://travis-ci.org/xitep/lines-rs)
[![](http://meritbadge.herokuapp.com/lines)](https://crates.io/crates/lines)

`lines` is a small library to efficiently parse text files line by
line while avoiding unnecessary memory allocations.

Typically, processing log files or other line oriented file formats,
doesn't always require allocation of memory for each processed line.
Instead, we can ...

1. ... allocate and maintain one buffer ...
2. ... locate a line in the processed stream and copy that to the mentioned
   buffer ...
3. ... lend the buffer to client code for "some number crunching," and ...
4. ... afterwards repeat locating the next line in the input at step 2.

By re-using a buffer and having the client decide whether or not to
make a copy of the read line, we can gain significant performance wins
in certain situations.

Usage
-----

Since `lines` uses [Cargo](http://crates.io), adding a dependency
section as follows should suffice to start using it:

```toml
[dependencies.lines]
version = "*"
```

The typical example of iterating a file line by line can be
demonstrated with the following program:

```rust
#[macro_use(read_lines)]
extern crate lines;

use lines::linereader::LineReader;
use std::fs::File;
use std::str::from_utf8;

fn main() {
    let f = File::open("main.rs").unwrap();
    read_lines!(line in LineReader::new(f), {
        let line = from_utf8(line.unwrap()).unwrap();
        print!("{}", line);
    });
}

```

There are certain limitations to the data that the library can
process.  Namely, a newline is assumed to be defined by '\n'.  More
information can be found in the [generated
documentation](https://docs.rs/lines/) of the library.
