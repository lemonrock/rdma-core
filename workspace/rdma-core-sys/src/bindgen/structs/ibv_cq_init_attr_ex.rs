// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_cq_init_attr_ex
{
	pub cqe: u32,
	pub cq_context: *mut c_void,
	pub channel: *mut ibv_comp_channel,
	pub comp_vector: u32,
	pub wc_flags: u64,
	pub comp_mask: u32,
	pub flags: u32,
}

impl Default for ibv_cq_init_attr_ex
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_cq_init_attr_ex
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_cq_init_attr_ex {{ cq_context: {:?}, channel: {:?} }}", self.cq_context, self.channel)
	}
}
