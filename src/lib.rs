#![feature(min_const_generics)]
#![feature(maybe_uninit_ref)]
#![cfg_attr(not(feature = "std"), no_std)]

mod buffered;
mod cursor;
mod error;
mod r#impl;
mod impls;
mod util;

#[cfg(not(feature = "std"))]
pub use cursor::Cursor;
#[cfg(not(feature = "std"))]
pub use error::{Error, ErrorKind, Result};
#[cfg(not(feature = "std"))]
pub use r#impl::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};
#[cfg(not(feature = "std"))]
pub use util::copy;

#[cfg(feature = "std")]
pub use std::io::{
    copy, BufRead, Bytes, Chain, Cursor, Error, ErrorKind, Read, Result, Seek, SeekFrom, Take,
    Write,
};

// Use this crate's implementation on both std and no_std
pub use buffered::{BufReader, BufWriter, LineWriter};
