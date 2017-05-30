// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub union rdma_cm_event__bindgen_ty_1
{
    pub conn: rdma_conn_param,
    pub ud: rdma_ud_param,
}

impl Clone for rdma_cm_event__bindgen_ty_1
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for rdma_cm_event__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Clone for rdma_cm_event
{
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for rdma_cm_event
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
