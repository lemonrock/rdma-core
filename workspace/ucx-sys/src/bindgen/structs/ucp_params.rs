// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ucp_params
{
	pub field_mask: u64,
	pub features: u64,
	pub request_size: usize,
	pub request_init: ucp_request_init_callback_t,
	pub request_cleanup: ucp_request_cleanup_callback_t,
	pub tag_sender_mask: u64,
	pub mt_workers_shared: c_int,
	pub estimated_num_eps: usize,
}

impl Clone for ucp_params
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ucp_params
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
