// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct ibv_global_route
{
	pub dgid: ibv_gid,
	pub flow_label: u32,
	pub sgid_index: u8,
	pub hop_limit: u8,
	pub traffic_class: u8,
}

impl Clone for ibv_global_route
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_global_route
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
