// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_shm_ep_opts
{
	pub tl: mxm_tl_ep_opts_t,
	pub rx: mxm_shm_ep_opts__bindgen_ty_1,
	pub fifo_size: c_uint,
	pub write_retry_count: c_uint,
	pub read_retry_count: c_uint,
	pub knem_max_simultaneous: c_uint,
	pub dma_chunk_size: usize,
	pub is_using_knem_dma: c_int,
	pub release_fifo_factor: f64,
	pub hugetlb_mode: mxm_ternary_value_t,
}
