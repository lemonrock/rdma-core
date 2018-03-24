// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_rdma_freeaddrinfo"] pub fn rdma_freeaddrinfo(res: *mut rdma_addrinfo);
	#[link_name = "\u{1}_rdma_getaddrinfo"] pub fn rdma_getaddrinfo(node: *const c_char, service: *const c_char, hints: *const rdma_addrinfo, res: *mut *mut rdma_addrinfo) -> c_int;
}
