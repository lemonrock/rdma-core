# mxm-reference-sys

This is a MIT licensed rust crate. It provides a FFI binding to the MXM 2 library C headers. We do not bundle `libmxm`.

`libmxm` is supplied by Mellanox in source code form and only works with their version of `libibverbs` and `glibc`. It can not work with `musl` as it uses backtrace functionality. It also links to `libz` (zlib). It _does not_ need `librdmacm`.

## Usage of libibverbs

It is interesting to note that MXM does not use many functions in `libibverbs`:-

```bash
x86_64-linux-musl-readelf -a libmxm.so.2.0.32 | less | grep 'UND' | grep 'UND ib' | awk '{print $8}' | awk -F'@' '{print $1}' | sort -u
ibv_ack_async_event
ibv_ack_cq_events
ibv_alloc_pd
ibv_close_device
ibv_create_ah
ibv_create_comp_channel
ibv_create_cq
ibv_create_qp
ibv_create_srq
ibv_dealloc_pd
ibv_dereg_mr
ibv_destroy_ah
ibv_destroy_comp_channel
ibv_destroy_cq
ibv_destroy_qp
ibv_destroy_srq
ibv_event_type_str
ibv_fork_init
ibv_free_device_list
ibv_get_async_event
ibv_get_cq_event
ibv_get_device_list
ibv_get_device_name
ibv_modify_qp
ibv_modify_srq
ibv_open_device
ibv_query_gid
ibv_query_port
ibv_wc_status_str
```

Note that this list can not include statically-inlined functions.
