// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_context_opts
{
	pub async_mode: mxm_async_mode_t,
	pub mem: mxm_context_opts__bindgen_ty_1,
	pub init_hook: *mut c_char,
	pub is_thread_single: c_int,
	pub shm_kcopy_mode: mxm_shm_kcopy_mode_t,
	pub ib: mxm_context_opts__bindgen_ty_2,
}
