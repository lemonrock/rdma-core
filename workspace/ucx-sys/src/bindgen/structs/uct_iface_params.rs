// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct uct_iface_params
{
	pub cpu_mask: ucs_cpu_set_t,
	pub tl_name: *const c_char,
	pub dev_name: *const c_char,
	pub stats_root: *mut ucs_stats_node_t,
	pub rx_headroom: usize,
	pub err_handler_arg: *mut c_void,
	pub err_handler: uct_error_handler_t,
	pub eager_arg: *mut c_void,
	pub eager_cb: uct_tag_unexp_eager_cb_t,
	pub rndv_arg: *mut c_void,
	pub rndv_cb: uct_tag_unexp_rndv_cb_t,
}

impl Clone for uct_iface_params
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for uct_iface_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
