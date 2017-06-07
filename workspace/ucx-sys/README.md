# ucx-sys

This is a MIT licensed rust crate. It provides a FFI binding to the [UCX](https://github.com/openucx/ucx) library. It is likely that it works better with the `libibverbs` library variant supplied by Mellanox.

ucx will not work on anything other than 64-bit x86-64 Linux. Upstream bugs currently prevent it from working with gcc 6.3.0 and the musl libc, although this should be fixed soon.

This binding is very much as work in progress.
