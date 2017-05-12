// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_mw_bind
{
	pub wr_id: u64,
	pub send_flags: c_int,
	pub bind_info: ibv_mw_bind_info,
}

impl Clone for ibv_mw_bind
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_mw_bind
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}