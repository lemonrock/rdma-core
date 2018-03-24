// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_send_wr__bindgen_ty_4__bindgen_ty_1
{
	pub mw: *mut ibv_mw,
	pub rkey: u32,
	pub bind_info: ibv_mw_bind_info,
}

impl Default for ibv_send_wr__bindgen_ty_4__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_send_wr__bindgen_ty_4__bindgen_ty_1
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_send_wr__bindgen_ty_4__bindgen_ty_1 {{ mw: {:?}, bind_info: {:?} }}", self.mw, self.bind_info)
	}
}
