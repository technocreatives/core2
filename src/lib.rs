#![cfg_attr(feature = "nightly", feature(min_const_generics))]
#![cfg_attr(feature = "nightly", feature(maybe_uninit_ref))]
#![cfg_attr(all(feature = "std", feature = "nightly"), feature(read_initializer))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "nightly")]
mod buffered;
mod cursor;
mod error;
mod impls;
mod traits;
mod util;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub use cursor::Cursor;
#[cfg(not(feature = "std"))]
pub use error::{Error, ErrorKind, Result};
#[cfg(not(feature = "std"))]
pub use traits::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};

#[cfg(feature = "std")]
pub use std::io::{
    BufRead, Bytes, Chain, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Take, Write,
};

// Use this crate's implementation on both std and no_std
#[cfg(feature = "nightly")]
pub use buffered::{BufReader, BufWriter, LineWriter};

#[cfg(feature = "nightly")]
pub use util::copy;
