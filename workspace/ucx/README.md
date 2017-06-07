# rdma-core

This is a MIT licensed rust crate. It provides a mid-level set of bindings over the FFI binding of UCX.

Pull requests are not likely to be accepted at this time.

ucx is MIT licensed.

ucx will not work on anything other than 64-bit x86-64 Linux. Upstream bugs currently prevent it from working with gcc 6.3.0 and the musl libc, although this should be fixed soon.

UCX, correctly, configured, uses libnuma (aka `numactl`). We do not correctly configure the build to use this at this time.
