// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_int_add(target: *mut c_int, value: c_int, pe: c_int);
	pub fn shmem_int_and_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
	pub fn shmem_int_cswap(target: *mut c_int, cond: c_int, value: c_int, pe: c_int) -> c_int;
	pub fn shmem_int_fadd(target: *mut c_int, value: c_int, pe: c_int) -> c_int;
	pub fn shmem_int_fetch(dest: *const c_int, pe: c_int) -> c_int;
	pub fn shmem_int_finc(target: *mut c_int, pe: c_int) -> c_int;
	pub fn shmem_int_g(addr: *mut c_int, pe: c_int) -> c_int;
	pub fn shmem_int_get(dest: *mut c_int, src: *const c_int, nelems: usize, pe: c_int);
	pub fn shmem_int_get_nbi(dest: *mut c_int, source: *const c_int, nelems: usize, pe: c_int);
	pub fn shmem_int_iget(target: *mut c_int, source: *const c_int, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_int_inc(target: *mut c_int, pe: c_int);
	pub fn shmem_int_iput(target: *mut c_int, source: *const c_int, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_int_max_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
	pub fn shmem_int_min_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
	pub fn shmem_int_or_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
	pub fn shmem_int_p(addr: *mut c_int, value: c_int, pe: c_int);
	pub fn shmem_int_prod_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
	pub fn shmem_int_put(dest: *mut c_int, src: *const c_int, nelems: usize, pe: c_int);
	pub fn shmem_int_put_nbi(dest: *mut c_int, source: *const c_int, nelems: usize, pe: c_int);
	pub fn shmem_int_set(dest: *mut c_int, value: c_int, pe: c_int);
	pub fn shmem_int_sum_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
	pub fn shmem_int_swap(target: *mut c_int, value: c_int, pe: c_int) -> c_int;
	pub fn shmem_int_wait(ivar: *mut c_int, cmp_value: c_int);
	pub fn shmem_int_wait_until(ivar: *mut c_int, cmp: c_int, cmp_value: c_int);
	pub fn shmem_int_xor_to_all(target: *mut c_int, source: *mut c_int, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_int, pSync: *mut c_long);
}
