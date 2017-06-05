// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ucm_event__bindgen_ty_4
{
	pub result: *mut c_void,
	pub shmid: c_int,
	pub shmaddr: *const c_void,
	pub shmflg: c_int,
}

impl Clone for ucm_event__bindgen_ty_4
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ucm_event__bindgen_ty_4
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
