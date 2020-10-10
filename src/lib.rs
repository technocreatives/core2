#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
mod error;
#[cfg(not(feature = "std"))]
mod r#impl;
#[cfg(not(feature = "std"))]
pub use error::{Error, ErrorKind, Result};
#[cfg(not(feature = "std"))]
pub use r#impl::{BufRead, Bytes, Chain, Read, Seek, SeekFrom, Take, Write};

#[cfg(feature = "std")]
pub use std::io::{
    BufRead, Bytes, Chain, Error, ErrorKind, Read, Result, Seek, SeekFrom, Take, Write,
};
