// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_free(ptr: *mut c_void);
	pub fn shmem_malloc(size: usize) -> *mut c_void;
	pub fn shmem_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}
