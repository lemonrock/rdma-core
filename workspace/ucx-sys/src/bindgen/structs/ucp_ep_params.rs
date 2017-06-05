// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ucp_ep_params
{
	pub field_mask: u64,
	pub address: *const ucp_address_t,
	pub err_mode: ucp_err_handling_mode_t,
	pub err_handler: ucp_err_handler_t,
}

impl Clone for ucp_ep_params
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ucp_ep_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
