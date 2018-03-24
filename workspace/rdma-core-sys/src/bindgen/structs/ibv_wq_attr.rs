// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_wq_attr
{
	pub attr_mask: u32,
	pub wq_state: ibv_wq_state,
	pub curr_wq_state: ibv_wq_state,
	pub flags: u32,
	pub flags_mask: u32,
}

impl Default for ibv_wq_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_wq_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_wq_attr {{ wq_state: {:?}, curr_wq_state: {:?} }}", self.wq_state, self.curr_wq_state)
	}
}
