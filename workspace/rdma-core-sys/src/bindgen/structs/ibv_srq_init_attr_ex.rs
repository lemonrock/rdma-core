// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_srq_init_attr_ex
{
	pub srq_context: *mut c_void,
	pub attr: ibv_srq_attr,
	pub comp_mask: u32,
	pub srq_type: ibv_srq_type,
	pub pd: *mut ibv_pd,
	pub xrcd: *mut ibv_xrcd,
	pub cq: *mut ibv_cq,
	pub tm_cap: ibv_tm_cap,
}

impl Default for ibv_srq_init_attr_ex
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_srq_init_attr_ex
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_srq_init_attr_ex {{ srq_context: {:?}, attr: {:?}, srq_type: {:?}, pd: {:?}, xrcd: {:?}, cq: {:?}, tm_cap: {:?} }}", self.srq_context, self.attr, self.srq_type, self.pd, self.xrcd, self.cq, self.tm_cap)
	}
}
