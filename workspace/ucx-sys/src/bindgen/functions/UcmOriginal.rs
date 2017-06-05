// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucm_orig_mmap(addr: *mut c_void, length: usize, prot: c_int, flags: c_int, fd: c_int, offset: off_t) -> *mut c_void;
	pub fn ucm_orig_mremap(old_address: *mut c_void, old_size: usize, new_size: usize, flags: c_int) -> *mut c_void;
	pub fn ucm_orig_munmap(addr: *mut c_void, length: usize) -> c_int;
	pub fn ucm_orig_sbrk(increment: isize) -> *mut c_void;
	pub fn ucm_orig_shmat(shmid: c_int, shmaddr: *const c_void, shmflg: c_int) -> *mut c_void;
	pub fn ucm_orig_shmdt(shmaddr: *const c_void) -> c_int;
}
