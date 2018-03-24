// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ibv_destroy_srq"] pub fn ibv_destroy_srq(srq: *mut ibv_srq) -> c_int;
	#[link_name = "\u{1}_ibv_modify_srq"] pub fn ibv_modify_srq(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr, srq_attr_mask: c_int) -> c_int;
	#[link_name = "\u{1}_ibv_query_srq"] pub fn ibv_query_srq(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr) -> c_int;
}
