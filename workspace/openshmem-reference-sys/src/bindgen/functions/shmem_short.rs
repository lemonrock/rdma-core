// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_short_and_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
	pub fn shmem_short_g(addr: *mut c_short, pe: c_int) -> c_short;
	pub fn shmem_short_get(dest: *mut c_short, src: *const c_short, nelems: usize, pe: c_int);
	pub fn shmem_short_get_nbi(dest: *mut c_short, source: *const c_short, nelems: usize, pe: c_int);
	pub fn shmem_short_iget(target: *mut c_short, source: *const c_short, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_short_iput(target: *mut c_short, source: *const c_short, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_short_max_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
	pub fn shmem_short_min_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
	pub fn shmem_short_or_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
	pub fn shmem_short_p(addr: *mut c_short, value: c_short, pe: c_int);
	pub fn shmem_short_prod_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
	pub fn shmem_short_put(dest: *mut c_short, src: *const c_short, nelems: usize, pe: c_int);
	pub fn shmem_short_put_nbi(dest: *mut c_short, source: *const c_short, nelems: usize, pe: c_int);
	pub fn shmem_short_sum_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
	pub fn shmem_short_wait(ivar: *mut c_short, cmp_value: c_short);
	pub fn shmem_short_wait_until(ivar: *mut c_short, cmp: c_int, cmp_value: c_short);
	pub fn shmem_short_xor_to_all(target: *mut c_short, source: *mut c_short, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut c_short, pSync: *mut c_long);
}
