// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union ibv_async_event__bindgen_ty_1
{
    pub cq: *mut ibv_cq,
    pub qp: *mut ibv_qp,
    pub srq: *mut ibv_srq,
    pub wq: *mut ibv_wq,
    pub port_num: c_int,
}

impl Clone for ibv_async_event__bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_async_event__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for ibv_async_event
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_async_event
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
