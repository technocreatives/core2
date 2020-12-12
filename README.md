# bare-io

[![Actions Status](https://github.com/bbqsrc/bare-io/workflows/CI/badge.svg)](https://github.com/bbqsrc/bare-io/actions)
[![Documentation](https://docs.rs/bare-io/badge.svg)](https://docs.rs/bare-io)
![Minimum Supported Rust Version (MSRV)](https://img.shields.io/badge/rust-v1.47.0+-blue)

Ever wanted a `Cursor` in `no_std`? Well now you can have it. A 'fork' of Rust's `std::io` module for `no_std` environments,
with the added benefit of not needing `alloc`.

The goal of this crate is to provide a stable interface for building I/O trait functionality in
`no_std` environments. The current code corresponds to the most recent stable API of Rust 1.47.0. 
It is also a goal to achieve a true alloc-less experience, with opt-in alloc support.

This crate works on `stable` with some limitations in functionality, and `nightly` without limitations by adding
the relevant feature flag.

This crate is `no_std` by default — you must opt into enabling `std` if required.

## Usage

```toml
[dependencies]
bare-io = "0.2"
```

Add the crate, use the things you would usually want from `std::io`, but instead from `bare_io`.

### Features

- **std**: enables `std` pass-throughs for the polyfilled types, but allows accessing the new types
- **alloc**: enable aspects of the `Read` and `Write` traits that require `alloc` support (WIP)
- **nightly**: enables **nightly**-only features, such as `BufReader` and `BufWriter` with const generic buffers.
- **nightly-std**: enables `std` with **nightly**-only features

### Differences to `std::io`

- No `std::io::Error`, so we have our own copy without any `Os` error functions
- `IoSlice` and the `*_vectored` family of functions are not implemented.
- `BufReader` and `BufWriter` have a different signature, as they now use a const generic bounded array for the internal buffer. (Requires **nightly** feature)

Other than items perhaps being entirely missing or certain functions unavailable on some traits, no function signatures have been changed.

### Limitations

- Using the buffer types currently requires **nightly** due to the use of const generics.
- Using `copy` or the buffer types with `std` support currently requires **nightly** due to the `initializer` API.

## Where is it used?

All of the below are works in progress, but should help with demonstrating how to use this crate.

- [fastvlq](https://github.com/bbqsrc/fastvlq): variable-length `u64` type with no-std reader support with `bare-io`
- [byteorder_bare-io](https://github.com/bbqsrc/byteorder): personal fork of `byteorder` crate
- [zstd-rs](https://github.com/bbqsrc/zstd-rs): personal fork of `zstd` crate to demonstrate `bare-io` in a more complex setting
- [comde](https://github.com/bbqsrc/comde): do compression/decompression similarly to how one might use `serde`
- [pathtrie](https://github.com/bbqsrc/pathtrie): a prefix trie optimised for paths and URLs, using an FST for fast lookups in no_std.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

---

Almost all of the code in this repository is a copy of the [Rust language codebase](https://github.com/rust-lang/rust) with minor modifications.

For attributions, see https://thanks.rust-lang.org/.
