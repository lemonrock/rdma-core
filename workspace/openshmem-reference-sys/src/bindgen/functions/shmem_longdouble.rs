// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_longdouble_g(addr: *mut f64, pe: c_int) -> f64;
	pub fn shmem_longdouble_get(dest: *mut f64, src: *const f64, nelems: usize, pe: c_int);
	pub fn shmem_longdouble_get_nbi(dest: *mut f64, source: *const f64, nelems: usize, pe: c_int);
	pub fn shmem_longdouble_iget(target: *mut f64, source: *const f64, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_longdouble_iput(target: *mut f64, source: *const f64, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_longdouble_max_to_all(target: *mut f64, source: *mut f64, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f64, pSync: *mut c_long);
	pub fn shmem_longdouble_min_to_all(target: *mut f64, source: *mut f64, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f64, pSync: *mut c_long);
	pub fn shmem_longdouble_p(addr: *mut f64, value: f64, pe: c_int);
	pub fn shmem_longdouble_prod_to_all(target: *mut f64, source: *mut f64, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f64, pSync: *mut c_long);
	pub fn shmem_longdouble_put(dest: *mut f64, src: *const f64, nelems: usize, pe: c_int);
	pub fn shmem_longdouble_put_nbi(dest: *mut f64, source: *const f64, nelems: usize, pe: c_int);
	pub fn shmem_longdouble_sum_to_all(target: *mut f64, source: *mut f64, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut f64, pSync: *mut c_long);
}
