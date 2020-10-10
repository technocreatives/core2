#![feature(min_const_generics)]
#![cfg_attr(not(feature = "std"), no_std)]

mod buffered;
mod cursor;
mod error;
mod r#impl;
mod impls;

#[cfg(not(feature = "std"))]
pub use cursor::Cursor;
#[cfg(not(feature = "std"))]
pub use error::{Error, ErrorKind, Result};
#[cfg(not(feature = "std"))]
pub use r#impl::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};

#[cfg(feature = "std")]
pub use std::io::{
    BufRead, Bytes, Chain, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Take, Write,
};

// Use this crate's implementation on both std and no_std
pub use buffered::{BufReader, BufWriter, LineWriter};
