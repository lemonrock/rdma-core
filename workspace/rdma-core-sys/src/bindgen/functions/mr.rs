// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ibv_dereg_mr"] pub fn ibv_dereg_mr(mr: *mut ibv_mr) -> c_int;
	#[link_name = "\u{1}_ibv_rereg_mr"] pub fn ibv_rereg_mr(mr: *mut ibv_mr, flags: c_int, pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> c_int;
}
