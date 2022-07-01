# dbs-fuse

The `dbs-fuse` is a utility crate to support [fuse-backend-rs](https://github.com/cloud-hypervisor/fuse-backend-rs).

### Wrappers for Rust async io

It's challenging to support Rust async io, and it's even more challenging to support Rust async io with Linux io-uring.

The `dbs-fuse` crate adds a wrapper layer over [tokio](https://tokio.rs/) and [tokio-uring](https://github.com/tokio-rs/tokio-uring) to simplify the way to support Rust async io by providing:
- FileVolatileSlice
- FileVolatileBuf

## License

This project is licensed under [Apache License](http://www.apache.org/licenses/LICENSE-2.0), Version 2.0.
