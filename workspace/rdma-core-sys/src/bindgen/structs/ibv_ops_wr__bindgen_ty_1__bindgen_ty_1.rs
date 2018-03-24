// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_ops_wr__bindgen_ty_1__bindgen_ty_1
{
	pub recv_wr_id: u64,
	pub sg_list: *mut ibv_sge,
	pub num_sge: c_int,
	pub tag: u64,
	pub mask: u64,
}

impl Default for ibv_ops_wr__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_ops_wr__bindgen_ty_1__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_ops_wr__bindgen_ty_1__bindgen_ty_1 {{ sg_list: {:?} }}", self.sg_list)
	}
}
