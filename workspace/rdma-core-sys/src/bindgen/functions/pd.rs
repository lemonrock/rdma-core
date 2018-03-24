// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ibv_create_ah"] pub fn ibv_create_ah(pd: *mut ibv_pd, attr: *mut ibv_ah_attr) -> *mut ibv_ah;
	#[link_name = "\u{1}_ibv_create_ah_from_wc"] pub fn ibv_create_ah_from_wc(pd: *mut ibv_pd, wc: *mut ibv_wc, grh: *mut ibv_grh, port_num: u8) -> *mut ibv_ah;
	#[link_name = "\u{1}_ibv_create_qp"] pub fn ibv_create_qp(pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> *mut ibv_qp;
	#[link_name = "\u{1}_ibv_create_srq"] pub fn ibv_create_srq(pd: *mut ibv_pd, srq_init_attr: *mut ibv_srq_init_attr) -> *mut ibv_srq;
	#[link_name = "\u{1}_ibv_dealloc_pd"] pub fn ibv_dealloc_pd(pd: *mut ibv_pd) -> c_int;
	#[link_name = "\u{1}_ibv_reg_mr"] pub fn ibv_reg_mr(pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> *mut ibv_mr;
}
