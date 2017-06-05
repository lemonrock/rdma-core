// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union pthread_barrier_t__bindgen_ty_1
{
    pub __i: [c_int; 8usize],
    pub __vi: [c_int; 8usize],
    pub __p: [*mut c_void; 4usize],
}

impl Clone for pthread_barrier_t__bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for pthread_barrier_t__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for pthread_barrier_t
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for pthread_barrier_t
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
