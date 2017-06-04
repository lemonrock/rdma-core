// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct mxm_cib_ep_opts__bindgen_ty_3
{
	pub srq_fill_size: c_uint,
	pub srq_resize_factor: f64,
}

impl Clone for mxm_cib_ep_opts__bindgen_ty_3
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Clone for mxm_cib_ep_opts
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for mxm_cib_ep_opts
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
