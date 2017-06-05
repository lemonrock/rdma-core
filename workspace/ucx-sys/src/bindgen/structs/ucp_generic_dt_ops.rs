// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ucp_generic_dt_ops
{
	pub start_pack: Option<unsafe extern "C" fn(context: *mut c_void, buffer: *const c_void, count: usize) -> *mut c_void>,
	pub start_unpack: Option<unsafe extern "C" fn(context: *mut c_void, buffer: *mut c_void, count: usize) -> *mut c_void>,
	pub packed_size: Option<unsafe extern "C" fn(state: *mut c_void) -> usize>,
	pub pack: Option<unsafe extern "C" fn(state: *mut c_void, offset: usize, dest: *mut c_void, max_length: usize) -> usize>,
	pub unpack: Option<unsafe extern "C" fn(state: *mut c_void, offset: usize, src: *const c_void, count: usize) -> ucs_status_t>,
	pub finish: Option<unsafe extern "C" fn(state: *mut c_void)>,
}

impl Clone for ucp_generic_dt_ops
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ucp_generic_dt_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
