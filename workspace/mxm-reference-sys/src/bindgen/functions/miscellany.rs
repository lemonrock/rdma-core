// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_cleanup(context: mxm_h);
	pub fn mxm_config_free_context_opts(opts: *mut mxm_context_opts_t);
	pub fn mxm_config_free_ep_opts(opts: *mut mxm_ep_opts_t);
	pub fn mxm_config_print(stream: *mut FILE, ctx_opts: *mut mxm_context_opts_t, ep_opts: *mut mxm_ep_opts_t, flags: c_uint);
	pub fn mxm_config_read_context_opts(optsp: *mut *mut mxm_context_opts_t) -> mxm_error_t;
	pub fn mxm_config_read_ep_opts(optsp: *mut *mut mxm_ep_opts_t) -> mxm_error_t;
	pub fn mxm_config_read_opts(ctx_optsp: *mut *mut mxm_context_opts_t, ep_optsp: *mut *mut mxm_ep_opts_t, prefix: *const c_char, config_file: *const c_char, flags: c_uint) -> mxm_error_t;
	pub fn mxm_conn_ctx_get(conn: mxm_conn_h) -> *mut c_void;
	pub fn mxm_conn_ctx_set(conn: mxm_conn_h, ctx: *mut c_void);
	pub fn mxm_ep_connect(ep: mxm_ep_h, address: *mut c_void, conn_p: *mut mxm_conn_h) -> mxm_error_t;
	pub fn mxm_ep_create(context: mxm_h, opts: *mut mxm_ep_opts_t, ep_p: *mut mxm_ep_h) -> mxm_error_t;
	pub fn mxm_ep_destroy(ep: mxm_ep_h);
	pub fn mxm_ep_disconnect(conn: mxm_conn_h) -> mxm_error_t;
	pub fn mxm_ep_get_address(ep: mxm_ep_h, address: *mut c_void, addrlen_p: *mut usize) -> mxm_error_t;
	pub fn mxm_ep_powerdown(ep: mxm_ep_h) -> mxm_error_t;
	pub fn mxm_ep_wireup(ep: mxm_ep_h) -> mxm_error_t;
	pub fn mxm_error_string(error: mxm_error_t) -> *const c_char;
	pub fn mxm_get_version() -> c_ulong;
	pub fn mxm_get_version_string() -> *const c_char;
	pub fn mxm_init(opts: *mut mxm_context_opts_t, context_p: *mut mxm_h) -> mxm_error_t;
	pub fn mxm_mem_get_key(context: mxm_h, address: *mut c_void, mkey: *mut mxm_mem_key_t) -> mxm_error_t;
	pub fn mxm_mem_map(context: mxm_h, address_p: *mut *mut c_void, length_p: *mut usize, flags: c_uint, remote_mkey: *mut mxm_mem_key_t, offset: usize) -> mxm_error_t;
	pub fn mxm_mem_unmap(context: mxm_h, address: *mut c_void, length: usize, flags: c_uint) -> mxm_error_t;
	pub fn mxm_message_recv(req: *mut mxm_recv_req_t, msg: mxm_message_h) -> mxm_error_t;
	pub fn mxm_message_release(msg: mxm_message_h) -> mxm_error_t;
	pub fn mxm_mq_create(context: mxm_h, ctxid: mxm_ctxid_t, mqp: *mut mxm_mq_h) -> mxm_error_t;
	pub fn mxm_mq_destroy(mq: mxm_mq_h);
	pub fn mxm_progress(context: mxm_h) -> mxm_error_t;
	pub fn mxm_progress_register(context: mxm_h, progress_cb: mxm_progress_cb_t, arg: *mut c_void) -> mxm_error_t;
	pub fn mxm_progress_unregister(context: mxm_h, progress_cb: mxm_progress_cb_t) -> mxm_error_t;
	pub fn mxm_req_cancel_recv(req: *mut mxm_recv_req_t) -> mxm_error_t;
	pub fn mxm_req_cancel_send(req: *mut mxm_send_req_t) -> mxm_error_t;
	pub fn mxm_req_mprobe(req: *mut mxm_recv_req_t, msgp: *mut mxm_message_h) -> mxm_error_t;
	pub fn mxm_req_probe(req: *mut mxm_recv_req_t) -> mxm_error_t;
	pub fn mxm_req_recv(req: *mut mxm_recv_req_t) -> mxm_error_t;
	pub fn mxm_req_send(req: *mut mxm_send_req_t) -> mxm_error_t;
	pub fn mxm_set_am_handler(context: mxm_h, hid: mxm_hid_t, cb: mxm_am_handler_t, flags: c_uint) -> mxm_error_t;
	pub fn mxm_wait(wait: *mut mxm_wait_t);
}
