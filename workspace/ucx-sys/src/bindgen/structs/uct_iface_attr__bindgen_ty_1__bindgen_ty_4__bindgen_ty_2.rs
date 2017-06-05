// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct uct_iface_attr__bindgen_ty_1__bindgen_ty_4__bindgen_ty_2
{
	pub max_short: usize,
	pub max_bcopy: usize,
	pub max_zcopy: usize,
	pub max_iov: usize,
}

impl Clone for uct_iface_attr__bindgen_ty_1__bindgen_ty_4__bindgen_ty_2
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}