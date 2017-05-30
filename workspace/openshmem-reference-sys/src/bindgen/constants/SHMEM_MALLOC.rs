// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub const SHMEM_MALLOC_ALREADY_FREE: c_int = -4;
pub const SHMEM_MALLOC_BAD_SIZE: c_int = -1;
pub const SHMEM_MALLOC_FAIL: c_int = -2;
pub const SHMEM_MALLOC_MEMALIGN_FAILED: c_int = -11;
pub const SHMEM_MALLOC_NOT_ALIGNED: c_int = -5;
pub const SHMEM_MALLOC_NOT_IN_SYMM_HEAP: c_int = -3;
pub const SHMEM_MALLOC_OK: c_uint = 0;
pub const SHMEM_MALLOC_REALLOC_FAILED: c_int = -12;
pub const SHMEM_MALLOC_SYMMSIZE_FAILED: c_int = -10;
