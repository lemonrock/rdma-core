// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub union rdma_addr__bindgen_ty_2
{
    pub dst_addr: sockaddr,
    pub dst_sin: sockaddr_in,
    pub dst_sin6: sockaddr_in6,
    pub dst_storage: sockaddr_storage,
}

impl Default for rdma_addr__bindgen_ty_2
{
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
