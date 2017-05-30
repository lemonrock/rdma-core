// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_addr_accessible(addr: *const c_void, pe: c_int) -> c_int;
	pub fn shmem_align(alignment: usize, size: usize) -> *mut c_void;
	pub fn shmem_alltoall32(target: *mut c_void, source: *const c_void, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_alltoall64(target: *mut c_void, source: *const c_void, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_alltoalls32(target: *mut c_void, source: *const c_void, dst: isize, sst: isize, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_alltoalls64(target: *mut c_void, source: *const c_void, dst: isize, sst: isize, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_barrier(PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_barrier_all();
	pub fn shmem_broadcast32(target: *mut c_void, source: *const c_void, nelems: usize, PE_root: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_broadcast64(target: *mut c_void, source: *const c_void, nelems: usize, PE_root: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_collect32(target: *mut c_void, source: *const c_void, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_collect64(target: *mut c_void, source: *const c_void, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_fcollect32(target: *mut c_void, source: *const c_void, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_fcollect64(target: *mut c_void, source: *const c_void, nelems: usize, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pSync: *mut c_long);
	pub fn shmem_fence();
	pub fn shmem_my_pe() -> c_int;
	pub fn shmem_n_pes() -> c_int;
	pub fn shmem_pe_accessible(pe: c_int) -> c_int;
	pub fn shmem_ptr(target: *const c_void, pe: c_int) -> *mut c_void;
	pub fn shmem_quiet();
}
