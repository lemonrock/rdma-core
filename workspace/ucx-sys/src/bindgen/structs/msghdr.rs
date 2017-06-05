// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct msghdr
{
	pub msg_name: *mut c_void,
	pub msg_namelen: socklen_t,
	pub msg_iov: *mut iovec,
	pub msg_iovlen: c_int,
	pub __pad1: c_int,
	pub msg_control: *mut c_void,
	pub msg_controllen: socklen_t,
	pub __pad2: socklen_t,
	pub msg_flags: c_int,
}

impl Clone for msghdr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for msghdr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
