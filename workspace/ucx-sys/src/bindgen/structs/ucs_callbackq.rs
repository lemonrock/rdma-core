// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ucs_callbackq
{
	pub start: *mut ucs_callbackq_elem_t,
	pub end: *mut ucs_callbackq_elem_t,
	pub ptr: *mut ucs_callbackq_elem_t,
	pub size: usize,
	pub slow_path: ucs_list_link_t,
	pub priv_: [c_char; 24usize],
}

impl Clone for ucs_callbackq
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ucs_callbackq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
