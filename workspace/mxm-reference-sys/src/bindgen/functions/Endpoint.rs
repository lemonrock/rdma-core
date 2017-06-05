// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_ep_connect(ep: mxm_ep_h, address: *mut c_void, conn_p: *mut mxm_conn_h) -> mxm_error_t;
	pub fn mxm_ep_create(context: mxm_h, opts: *mut mxm_ep_opts_t, ep_p: *mut mxm_ep_h) -> mxm_error_t;
	pub fn mxm_ep_destroy(ep: mxm_ep_h);
	pub fn mxm_ep_disconnect(conn: mxm_conn_h) -> mxm_error_t;
	pub fn mxm_ep_get_address(ep: mxm_ep_h, address: *mut c_void, addrlen_p: *mut usize) -> mxm_error_t;
	pub fn mxm_ep_powerdown(ep: mxm_ep_h) -> mxm_error_t;
	pub fn mxm_ep_wireup(ep: mxm_ep_h) -> mxm_error_t;
}
