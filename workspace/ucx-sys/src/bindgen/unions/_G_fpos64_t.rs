// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union _G_fpos64_t
{
    pub __opaque: [c_char; 16usize],
    pub __align: f64,
}

impl Clone for _G_fpos64_t
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for _G_fpos64_t
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
