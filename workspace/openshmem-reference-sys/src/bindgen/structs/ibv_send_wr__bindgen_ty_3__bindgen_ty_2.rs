// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_send_wr__bindgen_ty_3__bindgen_ty_2
{
	pub hdr: *mut c_void,
	pub hdr_sz: u16,
	pub mss: u16,
}

impl Clone for ibv_send_wr__bindgen_ty_3__bindgen_ty_2
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_send_wr__bindgen_ty_3__bindgen_ty_2
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for ibv_send_wr__bindgen_ty_3
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_send_wr__bindgen_ty_3
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for ibv_send_wr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_send_wr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
