// Copyright (C) 2022 Alibaba Cloud. All rights reserved.
//
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

//! A utility crate to support [fuse-backend-rs](https://github.com/cloud-hypervisor/fuse-backend-rs)
//!
//! ### Wrappers for Rust async io
//! It's challenging to support Rust async io, and it's even more challenging to support Rust async io with Linux io-uring.
//!
//! The `dbs-fuse` crate adds a wrapper layer over [tokio](https://tokio.rs/) and [tokio-uring](https://github.com/tokio-rs/tokio-uring) to simplify the way to support Rust async io by providing:
//! - [FileVolatileSlice](crate::buf::FileVolatileSlice): An adapter structure to work around limitations of the `vm-memory` crate.
//! - [FileVolatileBuf](crate::buf::FileVolatileBuf): An adapter structure to support `io-uring` based asynchronous IO.

pub mod buf;
