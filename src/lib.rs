// Copyright (C) 2022 Alibaba Cloud. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

//! A utility crate to support [fuse-backend-rs](https://github.com/cloud-hypervisor/fuse-backend-rs)
//!
//! ### Wrappers for Rust async io
//! It's challenging to support Rust async io, and it's even more challenging to support Rust async io with Linux io-uring.
//!
//! The `dbs-fuse` crate adds a wrapper layer over [tokio] and [tokio-uring] to simplify the way to support Rust async io by providing:
//! - [FileReadWriteVolatile]: A trait similar to `Read` and `Write`, but uses [FileVolatileSlice] objects as data buffers.
//! - [FileVolatileSlice]: An adapter structure to work around limitations of the `vm-memory` crate.
//! - [FileVolatileBuf]: An adapter structure to support `io-uring` based asynchronous IO.
//! - [File]: An adapter for for [tokio::fs::File] and [tokio-uring::fs::File].
//! - [Runtime]: An adapter for for [tokio::runtime::Runtime] and [tokio-uring::Runtime].
//!
//! [FileReadWriteVolatile]: crate::file_traits::FileReadWriteVolatile
//! [FileVolatileSlice]: crate::buf::FileVolatileSlice
//! [FileVolatileBuf]: crate::buf::FileVolatileBuf
//! [File]: crate::async_file::File
//! [Runtime]: crate::async_runtime::Runtime
//! [tokio]: https://tokio.rs/
//! [tokio-uring]: https://github.com/tokio-rs/tokio-uring

pub mod buf;
pub mod file_traits;

#[cfg(feature = "async-io")]
pub mod async_file;
#[cfg(feature = "async-io")]
pub mod async_runtime;
#[cfg(feature = "async-io")]
pub mod mpmc;

// Temporarily include all source code tokio-uring.
// Will switch to upstream once our enhancement have been merged and new version available.
#[cfg(all(feature = "async-io", target_os = "linux"))]
pub mod tokio_uring;
#[cfg(all(feature = "async-io", target_os = "linux"))]
use self::tokio_uring::{driver, fs, future, BufResult};

#[cfg(target_os = "linux")]
pub use libc::{off64_t, pread64, preadv64, pwrite64, pwritev64};
#[cfg(target_os = "macos")]
pub use libc::{
    off_t as off64_t, pread as pread64, preadv as preadv64, pwrite as pwrite64,
    pwritev as pwritev64,
};
