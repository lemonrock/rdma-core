// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub union ibv_wc__bindgen_ty_1
{
	pub imm_data: __be32,
	pub invalidated_rkey: u32,
	_bindgen_union_align: u32,
}

impl Default for ibv_wc__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_wc__bindgen_ty_1
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_wc__bindgen_ty_1 {{ union }}")
	}
}
