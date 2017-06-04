// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_context_opts__bindgen_ty_2
{
	pub ports: mxm_ib_port_configs_t,
	pub fork_safe: c_int,
	pub hugetlb_safe: c_int,
	pub hw_atomic_algo: c_int,
	pub odp: mxm_context_opts__bindgen_ty_2__bindgen_ty_1,
}
