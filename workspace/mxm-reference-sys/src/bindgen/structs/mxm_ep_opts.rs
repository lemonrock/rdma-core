// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_ep_opts
{
	pub ep_name: *mut c_char,
	pub ep_name_max: c_int,
	pub tl_bitmap: c_uint,
	pub tm: mxm_ep_opts__bindgen_ty_1,
	pub preconnect: c_int,
	pub self_: mxm_self_ep_opts_t,
	pub shm: mxm_shm_ep_opts_t,
	pub oob: mxm_oob_ep_opts_t,
	pub ud: mxm_ud_ep_opts_t,
	pub rc: mxm_rc_ep_opts_t,
	pub dc: mxm_dc_ep_opts_t,
}
