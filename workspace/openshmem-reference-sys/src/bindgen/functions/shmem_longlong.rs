// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_longlong_add(target: *mut c_longlong, value: c_longlong, pe: c_int);
	pub fn shmem_longlong_and_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
	pub fn shmem_longlong_cswap(target: *mut c_longlong, cond: c_longlong, value: c_longlong, pe: c_int) -> c_longlong;
	pub fn shmem_longlong_fadd(target: *mut c_longlong, value: c_longlong, pe: c_int) -> c_longlong;
	pub fn shmem_longlong_fetch(dest: *const c_longlong, pe: c_int) -> c_longlong;
	pub fn shmem_longlong_finc(target: *mut c_longlong, pe: c_int) -> c_longlong;
	pub fn shmem_longlong_g(addr: *mut c_longlong, pe: c_int) -> c_longlong;
	pub fn shmem_longlong_get(dest: *mut c_longlong, src: *const c_longlong, nelems: usize, pe: c_int);
	pub fn shmem_longlong_get_nbi(dest: *mut c_longlong, source: *const c_longlong, nelems: usize, pe: c_int);
	pub fn shmem_longlong_iget(target: *mut c_longlong, source: *const c_longlong, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_longlong_inc(target: *mut c_longlong, pe: c_int);
	pub fn shmem_longlong_iput(target: *mut c_longlong, source: *const c_longlong, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_longlong_max_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
	pub fn shmem_longlong_min_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
	pub fn shmem_longlong_or_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
	pub fn shmem_longlong_p(addr: *mut c_longlong, value: c_longlong, pe: c_int);
	pub fn shmem_longlong_prod_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
	pub fn shmem_longlong_put(dest: *mut c_longlong, src: *const c_longlong, nelems: usize, pe: c_int);
	pub fn shmem_longlong_put_nbi(dest: *mut c_longlong, source: *const c_longlong, nelems: usize, pe: c_int);
	pub fn shmem_longlong_set(dest: *mut c_longlong, value: c_longlong, pe: c_int);
	pub fn shmem_longlong_sum_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
	pub fn shmem_longlong_swap(target: *mut c_longlong, value: c_longlong, pe: c_int) -> c_longlong;
	pub fn shmem_longlong_wait(ivar: *mut c_longlong, cmp_value: c_longlong);
	pub fn shmem_longlong_wait_until(ivar: *mut c_longlong, cmp: c_int, cmp_value: c_longlong);
	pub fn shmem_longlong_xor_to_all(target: *mut c_longlong, source: *mut c_longlong, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_longlong, pSync: *mut c_long);
}
