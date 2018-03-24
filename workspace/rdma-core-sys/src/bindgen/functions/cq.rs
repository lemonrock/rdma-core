// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "\u{1}_ibv_ack_cq_events"] pub fn ibv_ack_cq_events(cq: *mut ibv_cq, nevents: c_uint);
	#[link_name = "\u{1}_ibv_destroy_cq"] pub fn ibv_destroy_cq(cq: *mut ibv_cq) -> c_int;
	#[link_name = "\u{1}_ibv_resize_cq"] pub fn ibv_resize_cq(cq: *mut ibv_cq, cqe: c_int) -> c_int;
}
