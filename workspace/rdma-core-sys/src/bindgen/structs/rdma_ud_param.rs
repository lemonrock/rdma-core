// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct rdma_ud_param
{
	pub private_data: *const c_void,
	pub private_data_len: u8,
	pub ah_attr: ibv_ah_attr,
	pub qp_num: u32,
	pub qkey: u32,
}

impl Default for rdma_ud_param
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rdma_ud_param
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rdma_ud_param {{ private_data: {:?}, ah_attr: {:?} }}", self.private_data, self.ah_attr)
	}
}
