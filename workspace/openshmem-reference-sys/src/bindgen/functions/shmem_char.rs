// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn shmem_char_g(addr: *mut c_char, pe: c_int) -> c_char;
	pub fn shmem_char_get(dest: *mut c_char, src: *const c_char, nelems: usize, pe: c_int);
	pub fn shmem_char_get_nbi(dest: *mut c_char, source: *const c_char, nelems: usize, pe: c_int);
	pub fn shmem_char_iget(target: *mut c_char, source: *const c_char, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_char_iput(target: *mut c_char, source: *const c_char, tst: isize, sst: isize, nelems: usize, pe: c_int);
	pub fn shmem_char_p(addr: *mut c_char, value: c_char, pe: c_int);
	pub fn shmem_char_put(dest: *mut c_char, src: *const c_char, nelems: usize, pe: c_int);
	pub fn shmem_char_put_nbi(dest: *mut c_char, source: *const c_char, nelems: usize, pe: c_int);
}
