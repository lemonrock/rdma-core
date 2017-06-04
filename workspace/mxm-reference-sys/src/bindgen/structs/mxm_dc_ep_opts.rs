// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_dc_ep_opts
{
	pub cib: mxm_cib_ep_opts_t,
	pub use_nop: c_int,
	pub rdma_qp_limit: c_uint,
	pub tx_policy: mxm_dc_tx_policy_t,
	pub dcs: mxm_dc_ep_opts__bindgen_ty_1,
}
