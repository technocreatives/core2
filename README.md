# bare-io

A 'fork' of Rust's `std::io` module for `no_std` environments, with the added benefit of not needing `alloc`.

The goal of this crate is to provide a stable interface for building I/O trait functionality in
`no_std` environments. The current code corresponds to the most recent stable API of Rust 1.47.0.

**This crate currently only works on nightly due to using const generics.**

### Usage

Add the crate, use the things you would usually want from `std::io`, but instead from `bare_io`.

Add the `std` feature to enable `std` compatibility by falling through to the `std` traits.

### Differences to `std::io`

- No `std::io::Error`, so we have our own copy without any `Os` error functions
- `IoSlice` and the `*_vectored` family of functions are not implemented.
- `BufReader` and `BufWriter` have a different signature, as they now use a const generic bounded array for the internal buffer.

Other than items perhaps being entirely missing or certain functions unavailable on some traits, no function signatures have been changed.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

Almost all of the code in this repository is a copy of the [Rust language codebase](https://github.com/rust-lang/rust) with minor modifications.

For attributions, see https://thanks.rust-lang.org/.