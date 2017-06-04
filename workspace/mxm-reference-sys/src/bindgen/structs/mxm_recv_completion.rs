// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_recv_completion
{
	pub source: mxm_conn_h,
	pub sender_tag: mxm_tag_t,
	pub sender_imm: mxm_imm_t,
	pub sender_len: usize,
	pub actual_len: usize,
}

impl Clone for mxm_recv_completion
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for mxm_recv_completion
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
