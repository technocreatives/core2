#![cfg_attr(feature = "nightly", feature(never_type))]
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "std", allow(dead_code))]
#[rustversion::attr(all(before(1.55), nightly), feature(maybe_uninit_ref))]
#[rustversion::attr(all(before(1.55), nightly), feature(read_initializer))]

#[cfg(not(feature = "std"))]
pub mod error;

#[cfg(feature = "std")]
pub use std::error as error;

#[cfg(not(feature = "std"))]
pub mod io;

#[cfg(feature = "std")]
pub use std::io as io;

#[cfg(feature = "alloc")]
extern crate alloc;
