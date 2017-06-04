// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_ib_ep_opts
{
	pub tl: mxm_tl_ep_opts_t,
	pub map_mode: mxm_ib_map_mode_t,
	pub drain_cq: c_int,
	pub cq_wmark: c_uint,
	pub resize_cq: c_int,
	pub first_sl: c_int,
	pub num_sls: c_uint,
	pub int_mode: c_uint,
	pub int_thresh: f64,
	pub exp_connectib: mxm_ternary_value_t,
	pub wc_mode: c_uint,
	pub cq_stall: mxm_ternary_value_t,
	pub cq_stall_loops: c_uint,
	pub lid_path: mxm_ib_ep_opts__bindgen_ty_1,
	pub max_path_bits: c_uint,
	pub lid_path_policy: mxm_ib_lid_path_policy_t,
	pub rx: mxm_ib_ep_opts__bindgen_ty_2,
	pub tx: mxm_ib_ep_opts__bindgen_ty_3,
	pub min_chunk: c_int,
	pub gid_index: c_uint,
	pub use_grh: mxm_ternary_value_t,
}
