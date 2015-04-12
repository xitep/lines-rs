//! Utilities for iterating readers efficiently line-by-line over
//! ASCII or UTF-8 encoded text content.  Lines are identified by a
//! LINE-FEED, i.e. a byte with the value ```\x0A```.  Content at the
//! end of a stream will be considered as a line - no matter whether
//! it was termined by a line-feed or not.
//!
//! Efficiency is achieved by two limitations:
//!
//!   * The provided utilities avoid allocation of memory for each
//!   identified line by reusing internal buffers.  Clients are
//!   supposed to make their own copy of a line if it needs to be
//!   remembered for later use in the program.
//!
//!   * The provided utilities do not validate proper encoding of the
//!   input data, and leave this up to the client.

mod bytes;
pub mod linemapper;
pub mod linereader;
