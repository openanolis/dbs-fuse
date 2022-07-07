# dbs-fuse

The `dbs-fuse` is a utility crate to support [fuse-backend-rs](https://github.com/cloud-hypervisor/fuse-backend-rs).

### Wrappers for Rust async io

It's challenging to support Rust async io, and it's even more challenging to support Rust async io with Linux io-uring.

The `dbs-fuse` crate adds a wrapper layer over [tokio](https://tokio.rs/) and [tokio-uring](https://github.com/tokio-rs/tokio-uring) to simplify the way to support Rust async io by providing:
- [FileReadWriteVolatile](https://docs.rs/dbs-fuse/latest/dbs_fuse/file_traits/trait.FileReadWriteVolatile.html): A trait similar to `Read` and `Write`, but uses [FileVolatileSlice] objects as data buffers.
- [FileVolatileSlice](https://docs.rs/dbs-fuse/latest/dbs_fuse/buf/struct.FileVolatileSlice.html): An adapter structure to work around limitations of the `vm-memory` crate.
- [FileVolatileBuf](https://docs.rs/dbs-fuse/latest/dbs_fuse/buf/struct.FileVolatileBuf.html): An adapter structure to support `io-uring` based asynchronous IO.
- File
- Runtime

## License

This project is licensed under [Apache License](http://www.apache.org/licenses/LICENSE-2.0), Version 2.0.

Source code under [src/tokio-uring] is temporarily copied from [tokio-uring](https://github.com/tokio-rs/tokio-uring)
with modifications, which is licensed under [MIT](https://github.com/tokio-rs/tokio-uring/blob/master/LICENSE).
