// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_cib_ep_opts
{
	pub ib: mxm_ib_ep_opts_t,
	pub qp_limit: c_uint,
	pub path_mtu: mxm_ib_mtu_t,
	pub min_rnr_timer: c_uint,
	pub timeout: c_uint,
	pub max_rdma_dst_ops: c_uint,
	pub retry_count: c_uint,
	pub rnr_retry: c_uint,
	pub use_hw_atomics: c_int,
	pub eager_rdma: mxm_cib_ep_opts__bindgen_ty_1,
	pub tx: mxm_cib_ep_opts__bindgen_ty_2,
	pub rx: mxm_cib_ep_opts__bindgen_ty_3,
}
