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
	
	/// May actually be an ibv_cq_ex (as of April 2017, only Mellanox 4 & 5 drivers support these)
	/// May be the same pointer as self.receiveCompletionQueue()
	/// May be null?
	#[inline(always)]
	fn sendCompletionQueue(self) -> *mut ibv_cq;
	
	/// May actually be an ibv_cq_ex (as of April 2017, only Mellanox 4 & 5 drivers support these)
	/// May be the same pointer as self.sendCompletionQueue()
	/// May be null?
	#[inline(always)]
	fn receiveCompletionQueue(self) -> *mut ibv_cq;
	
	/// May be null?
	#[inline(always)]
	fn sharedReceiveQueue(self) -> *mut ibv_srq;
	
	#[inline(always)]
	fn state(self) -> ibv_qp_state;
	
	#[inline(always)]
	fn queuePairNumber(self) -> QueuePairNumber;
	
	#[inline(always)]
	fn queuePairType(self) -> ibv_qp_type;
	
	#[inline(always)]
	fn eventsCompleted(self) -> u32;
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
	
	#[inline(always)]
	fn sendCompletionQueue(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).send_cq }
	}
	
	#[inline(always)]
	fn receiveCompletionQueue(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).recv_cq }
	}
	
	/// May be null?
	#[inline(always)]
	fn sharedReceiveQueue(self) -> *mut ibv_srq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).srq }
	}
	
	#[inline(always)]
	fn state(self) -> ibv_qp_state
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).state }
	}
	
	#[inline(always)]
	fn queuePairNumber(self) -> QueuePairNumber
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_num }
	}
	
	#[inline(always)]
	fn queuePairType(self) -> ibv_qp_type
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).qp_type }
	}
	
	#[inline(always)]
	fn eventsCompleted(self) -> u32
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).events_completed }
	}
}
