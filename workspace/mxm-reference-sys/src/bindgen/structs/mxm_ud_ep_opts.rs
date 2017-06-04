// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_ud_ep_opts
{
	pub ib: mxm_ib_ep_opts_t,
	pub ack_timeout: f64,
	pub fast_ack_timeout: f64,
	pub fast_timer_res: f64,
	pub window_size: c_uint,
	pub ca: mxm_ud_ca_t,
	pub ca_low_window: c_int,
	pub chk_max_size: c_uint,
	pub timeout: f64,
	pub rx: mxm_ud_ep_opts__bindgen_ty_1,
	pub zcopy_rndv: mxm_ud_ep_opts__bindgen_ty_2,
}
