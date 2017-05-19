// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_qpEx: HasContextPointer
{
	/// ibv_post_send - Post a list of work requests to a send queue.
	/// If IBV_SEND_INLINE flag is set, the data buffers can be reused immediately after the call returns.
	#[inline(always)]
	fn ibv_post_send(self, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr) -> c_int;
	
	/// ibv_post_recv - Post a list of work requests to a receive queue.
	#[inline(always)]
	fn ibv_post_recv(self, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr) -> c_int;
}

impl ibv_qpEx for *mut ibv_qp
{
	/// ibv_post_send - Post a list of work requests to a send queue.
	/// If IBV_SEND_INLINE flag is set, the data buffers can be reused immediately after the call returns.
	#[inline(always)]
	fn ibv_post_send(self, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr) -> c_int
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!wr.is_null(), "wr is null");
		debug_assert!(!bad_wr.is_null(), "bad_wr is null");
		
		unsafe { self.verbs().ops().post_send.unwrap()(self, wr, bad_wr) }
	}
	
	/// ibv_post_recv - Post a list of work requests to a receive queue.
	#[inline(always)]
	fn ibv_post_recv(self, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr) -> c_int
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!wr.is_null(), "wr is null");
		debug_assert!(!bad_wr.is_null(), "bad_wr is null");
		
		unsafe { self.verbs().ops().post_recv.unwrap()(self, wr, bad_wr) }
	}
}
