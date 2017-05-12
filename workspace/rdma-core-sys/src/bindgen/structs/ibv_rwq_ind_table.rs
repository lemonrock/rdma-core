// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_rwq_ind_table
{
	pub context: *mut ibv_context,
	pub ind_tbl_handle: c_int,
	pub ind_tbl_num: c_int,
	pub comp_mask: u32,
}

impl Clone for ibv_rwq_ind_table
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_rwq_ind_table
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}