// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_qp_open_attr
{
	pub comp_mask: u32,
	pub qp_num: u32,
	pub xrcd: *mut ibv_xrcd,
	pub qp_context: *mut c_void,
	pub qp_type: ibv_qp_type,
}

impl Default for ibv_qp_open_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_qp_open_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_qp_open_attr {{ xrcd: {:?}, qp_context: {:?}, qp_type: {:?} }}", self.xrcd, self.qp_context, self.qp_type)
	}
}
