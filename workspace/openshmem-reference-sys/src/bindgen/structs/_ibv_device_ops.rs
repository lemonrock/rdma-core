// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct _ibv_device_ops
{
	pub _dummy1: Option<unsafe extern "C" fn(device: *mut ibv_device, cmd_fd: c_int) -> *mut ibv_context>,
	pub _dummy2: Option<unsafe extern "C" fn(context: *mut ibv_context)>,
}

impl Clone for _ibv_device_ops
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for _ibv_device_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
