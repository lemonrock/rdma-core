// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_disconnect_nb(ep: ucp_ep_h) -> ucs_status_ptr_t;
	pub fn ucp_ep_create(worker: ucp_worker_h, params: *const ucp_ep_params_t, ep_p: *mut ucp_ep_h) -> ucs_status_t;
	pub fn ucp_ep_destroy(ep: ucp_ep_h);
	pub fn ucp_ep_flush(ep: ucp_ep_h) -> ucs_status_t;
	pub fn ucp_ep_print_info(ep: ucp_ep_h, stream: *mut FILE);
	pub fn ucp_ep_rkey_unpack(ep: ucp_ep_h, rkey_buffer: *mut c_void, rkey_p: *mut ucp_rkey_h) -> ucs_status_t;
	pub fn ucp_rmem_ptr(ep: ucp_ep_h, remote_addr: *mut c_void, rkey: ucp_rkey_h, local_addr_p: *mut *mut c_void) -> ucs_status_t;
}
