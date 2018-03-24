// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub union rdma_addr__bindgen_ty_1
{
	pub src_addr: sockaddr,
	pub src_sin: sockaddr_in,
	pub src_sin6: sockaddr_in6,
	pub src_storage: sockaddr_storage,
	_bindgen_union_align: [u64; 16usize],
}

impl Default for rdma_addr__bindgen_ty_1
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for rdma_addr__bindgen_ty_1
{
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "rdma_addr__bindgen_ty_1 {{ union }}")
	}
}
