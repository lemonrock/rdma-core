// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn uct_mem_alloc(addr: *mut c_void, min_length: usize, flags: c_uint, methods: *mut uct_alloc_method_t, num_methods: c_uint, mds: *mut uct_md_h, num_mds: c_uint, name: *const c_char, mem: *mut uct_allocated_memory_t) -> ucs_status_t;
	pub fn uct_mem_free(mem: *const uct_allocated_memory_t) -> ucs_status_t;
}
