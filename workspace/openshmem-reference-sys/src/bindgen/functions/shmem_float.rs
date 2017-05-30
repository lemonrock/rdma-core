// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_float_fetch(dest: *const f32, pe: c_int) -> f32;
	pub fn shmem_float_g(addr: *mut f32, pe: c_int) -> f32;
	pub fn shmem_float_get(dest: *mut f32, src: *const f32, nelems: usize, pe: c_int);
	pub fn shmem_float_get_nbi(dest: *mut f32, source: *const f32, nelems: usize, pe: c_int);
	pub fn shmem_float_iget(target: *mut f32, source: *const f32, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_float_iput(target: *mut f32, source: *const f32, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_float_max_to_all(target: *mut f32, source: *mut f32, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f32, pSync: *mut c_long);
	pub fn shmem_float_min_to_all(target: *mut f32, source: *mut f32, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f32, pSync: *mut c_long);
	pub fn shmem_float_p(addr: *mut f32, value: f32, pe: c_int);
	pub fn shmem_float_prod_to_all(target: *mut f32, source: *mut f32, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f32, pSync: *mut c_long);
	pub fn shmem_float_put(dest: *mut f32, src: *const f32, nelems: usize, pe: c_int);
	pub fn shmem_float_put_nbi(dest: *mut f32, source: *const f32, nelems: usize, pe: c_int);
	pub fn shmem_float_set(dest: *mut f32, value: f32, pe: c_int);
	pub fn shmem_float_sum_to_all(target: *mut f32, source: *mut f32, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f32, pSync: *mut c_long);
	pub fn shmem_float_swap(target: *mut f32, value: f32, pe: c_int) -> f32;
}
