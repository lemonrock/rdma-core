// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub union epoll_data
{
	pub ptr: *mut c_void,
	pub fd: c_int,
	pub u32: u32,
	pub u64: u64,
	_bindgen_union_align: u64,
}

impl Default for epoll_data
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for epoll_data
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "epoll_data {{ union }}")
	}
}
