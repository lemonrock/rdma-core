// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct rdma_ib_addr
{
	pub sgid: ibv_gid,
	pub dgid: ibv_gid,
	pub pkey: __be16,
}

impl Clone for rdma_ib_addr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for rdma_ib_addr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
