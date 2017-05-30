// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_qp_open_attr
{
	pub comp_mask: u32,
	pub qp_num: u32,
	pub xrcd: *mut ibv_xrcd,
	pub qp_context: *mut c_void,
	pub qp_type: ibv_qp_type,
}

impl Clone for ibv_qp_open_attr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_qp_open_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl ::core::ops::BitOr<ibv_qp_attr_mask> for ibv_qp_attr_mask
{
	type Output = Self;
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		ibv_qp_attr_mask(self.0 | other.0)
	}
}
