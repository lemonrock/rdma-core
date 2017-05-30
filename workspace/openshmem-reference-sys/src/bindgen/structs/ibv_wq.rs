// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Copy)]
pub struct ibv_wq
{
	pub context: *mut ibv_context,
	pub wq_context: *mut c_void,
	pub pd: *mut ibv_pd,
	pub cq: *mut ibv_cq,
	pub wq_num: u32,
	pub handle: u32,
	pub state: ibv_wq_state,
	pub wq_type: ibv_wq_type,
	pub post_recv: Option<unsafe extern "C" fn(current: *mut ibv_wq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int>,
	pub mutex: pthread_mutex_t,
	pub cond: pthread_cond_t,
	pub events_completed: u32,
	pub comp_mask: u32,
}

impl Clone for ibv_wq
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_wq
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
