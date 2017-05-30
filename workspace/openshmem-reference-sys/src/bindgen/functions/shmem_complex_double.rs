// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_complexd_prod_to_all(target: *mut __BindgenComplex<f64>, source: *mut __BindgenComplex<f64>, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut __BindgenComplex<f64>, pSync: *mut c_long);
	pub fn shmem_complexd_sum_to_all(target: *mut __BindgenComplex<f64>, source: *mut __BindgenComplex<f64>, nreduce: c_int, PE_start: c_int, logPE_stride: c_int, PE_size: c_int, pWrk: *mut __BindgenComplex<f64>, pSync: *mut c_long);
}
