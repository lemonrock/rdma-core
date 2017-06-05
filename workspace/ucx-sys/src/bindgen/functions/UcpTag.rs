// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_tag_msg_recv_nb(worker: ucp_worker_h, buffer: *mut c_void, count: usize, datatype: ucp_datatype_t, message: ucp_tag_message_h, cb: ucp_tag_recv_callback_t) -> ucs_status_ptr_t;
	pub fn ucp_tag_probe_nb(worker: ucp_worker_h, tag: ucp_tag_t, tag_mask: ucp_tag_t, remove: c_int, info: *mut ucp_tag_recv_info_t) -> ucp_tag_message_h;
	pub fn ucp_tag_recv_nb(worker: ucp_worker_h, buffer: *mut c_void, count: usize, datatype: ucp_datatype_t, tag: ucp_tag_t, tag_mask: ucp_tag_t, cb: ucp_tag_recv_callback_t) -> ucs_status_ptr_t;
	pub fn ucp_tag_recv_nbr(worker: ucp_worker_h, buffer: *mut c_void, count: usize, datatype: ucp_datatype_t, tag: ucp_tag_t, tag_mask: ucp_tag_t, req: *mut c_void) -> ucs_status_t;
	pub fn ucp_tag_send_nb(ep: ucp_ep_h, buffer: *const c_void, count: usize, datatype: ucp_datatype_t, tag: ucp_tag_t, cb: ucp_send_callback_t) -> ucs_status_ptr_t;
	pub fn ucp_tag_send_sync_nb(ep: ucp_ep_h, buffer: *const c_void, count: usize, datatype: ucp_datatype_t, tag: ucp_tag_t, cb: ucp_send_callback_t) -> ucs_status_ptr_t;
}
