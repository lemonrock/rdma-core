// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct uct_tl_resource_desc
{
	pub tl_name: [c_char; 10usize],
	pub dev_name: [c_char; 32usize],
	pub dev_type: uct_device_type_t,
}

impl Clone for uct_tl_resource_desc
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for uct_tl_resource_desc
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
