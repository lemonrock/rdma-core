// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_mem_get_key(context: mxm_h, address: *mut c_void, mkey: *mut mxm_mem_key_t) -> mxm_error_t;
	pub fn mxm_mem_map(context: mxm_h, address_p: *mut *mut c_void, length_p: *mut usize, flags: c_uint, remote_mkey: *mut mxm_mem_key_t, offset: usize) -> mxm_error_t;
	pub fn mxm_mem_unmap(context: mxm_h, address: *mut c_void, length: usize, flags: c_uint) -> mxm_error_t;
}
