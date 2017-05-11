// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ibv_create_ah(pd: *mut ibv_pd, attr: *mut ibv_ah_attr) -> *mut ibv_ah;
	pub fn ibv_create_ah_from_wc(pd: *mut ibv_pd, wc: *mut ibv_wc, grh: *mut ibv_grh, port_num: u8) -> *mut ibv_ah;
	pub fn ibv_reg_mr(pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> *mut ibv_mr;
}
