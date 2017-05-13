// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union ibv_wc__bindgen_ty_1
{
    pub imm_data: __be32,
    pub invalidated_rkey: u32,
}

impl Clone for ibv_wc__bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_wc__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for ibv_wc
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_wc
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl ::core::ops::BitOr<ibv_access_flags> for ibv_access_flags
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		ibv_access_flags(self.0 | other.0)
	}
}
