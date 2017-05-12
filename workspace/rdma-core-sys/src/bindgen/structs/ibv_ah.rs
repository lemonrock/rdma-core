// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_ah
{
	pub context: *mut ibv_context,
	pub pd: *mut ibv_pd,
	pub handle: u32,
}

impl Clone for ibv_ah
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_ah
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}