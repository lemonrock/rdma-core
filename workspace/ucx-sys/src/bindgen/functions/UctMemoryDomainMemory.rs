// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn uct_md_mem_advise(md: uct_md_h, memh: uct_mem_h, addr: *mut c_void, length: usize, advice: uct_mem_advice_t) -> ucs_status_t;
	pub fn uct_md_mem_alloc(md: uct_md_h, length_p: *mut usize, address_p: *mut *mut c_void, flags: c_uint, name: *const c_char, memh_p: *mut uct_mem_h) -> ucs_status_t;
	pub fn uct_md_mem_dereg(md: uct_md_h, memh: uct_mem_h) -> ucs_status_t;
	pub fn uct_md_mem_free(md: uct_md_h, memh: uct_mem_h) -> ucs_status_t;
	pub fn uct_md_mem_reg(md: uct_md_h, address: *mut c_void, length: usize, flags: c_uint, memh_p: *mut uct_mem_h) -> ucs_status_t;
}
