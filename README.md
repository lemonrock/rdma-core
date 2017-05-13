# rdma-core

rdma-core is a project to develop rust code to wrap the rdma-core C libraries. Currently it provides:-

* `rdma-core-sys`, a Rust crate that provides a low-level FFI binding of `libibverbs`, as of version `r14-rc1`
* `rdma-core`, an incomplete mid-level binding that uses `rdma-core-sys`, `rdma`

The `rdma-core-sys` crate will compile on Alpine Linux and Mac OS X, but bindings generation has only been tested on Mac OS X. `rdma-core-sys` also includes generated bindings to static inline functions present in `libiberbs`. Please note that there is another Rust project to provide rdma-core bindings, <https://github.com/jonhoo/rust-ibverbs>.

This project exists currently in an experimental form and primarily is being developed to serve the development needs of StormMQ. It may never stablise or support a development ecosystem that you are using. It also uses naming and code layout conventions that we prefer and are not accepted by the rust community. It requires Rust nightly, and likely will do so for some time (it uses unions, associated constants and the like). It prefers shell script over Cargo build scripts. Eventually, it is planned to incorporate it into [Libertine Linux](https://github.com/libertine-linux), a small, in-memory, secure and built-from-source Linux distribution that is ideal for modern microservice clusters.

Crates are present in the `workspace` folder along with IntelliJ module files.

## Licensing

The license for this project is MIT.
