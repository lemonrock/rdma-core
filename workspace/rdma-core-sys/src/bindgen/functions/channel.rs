// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ibv_destroy_comp_channel(channel: *mut ibv_comp_channel) -> c_int;
	pub fn ibv_get_cq_event(channel: *mut ibv_comp_channel, cq: *mut *mut ibv_cq, cq_context: *mut *mut c_void) -> c_int;
}
