// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_wq_init_attr
{
	pub wq_context: *mut c_void,
	pub wq_type: ibv_wq_type,
	pub max_wr: u32,
	pub max_sge: u32,
	pub pd: *mut ibv_pd,
	pub cq: *mut ibv_cq,
	pub comp_mask: u32,
	pub create_flags: u32,
}

impl Default for ibv_wq_init_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_wq_init_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_wq_init_attr {{ wq_context: {:?}, wq_type: {:?}, pd: {:?}, cq: {:?} }}", self.wq_context, self.wq_type, self.pd, self.cq)
	}
}
