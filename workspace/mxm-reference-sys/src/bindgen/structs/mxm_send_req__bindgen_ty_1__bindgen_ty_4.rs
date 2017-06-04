// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_send_req__bindgen_ty_1__bindgen_ty_4
{
	pub remote_vaddr: mxm_vaddr_t,
	pub remote_mkey: *mut mxm_mem_key_t,
	pub value: u64,
	pub order: u8,
}

impl Clone for mxm_send_req__bindgen_ty_1__bindgen_ty_4
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for mxm_send_req__bindgen_ty_1__bindgen_ty_4
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for mxm_send_req__bindgen_ty_1
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for mxm_send_req__bindgen_ty_1
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Default for mxm_send_req
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
