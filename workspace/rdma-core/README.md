# rdma-core

This is a MIT licensed rust crate. It provides a mid-level set of bindings over the FFI binding of rdma-core (`rdma-core-sys`). rdma-core is a set of libraries developed by the Linux developers, and includes a number of items. This binding supplies support for using libibverbs and librdmacm but not other libraries at this time with epoll. It will continue to change substantially, as its downstream users flesh out its interface.

Pull requests are not likely to be accepted at this time.

rdma-core is MIT licensed.

rdma-core will not work on anything other than 64-bit x86-64 Linux. It has been cross-compile tested only for the 'x86_64-unknown-linux-musl' target.
