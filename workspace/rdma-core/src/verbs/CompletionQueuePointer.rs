// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CompletionQueuePointer: Sized
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq;
	
	#[inline(always)]
	fn verbs(self) -> *mut ibv_context
	{
		unsafe { (*self.pointer()).context }
	}
	
	#[inline(always)]
	fn completionChannel(self) -> *mut ibv_comp_channel
	{
		unsafe { (*self.pointer()).channel }
	}
	
	#[inline(always)]
	fn context(self) -> *mut c_void
	{
		unsafe { (*self.pointer()).cq_context }
	}
	
	#[inline(always)]
	fn handle(self) -> u32
	{
		unsafe { (*self.pointer()).handle }
	}
	
	#[inline(always)]
	fn maximumNumberOfEntries(self) -> c_int
	{
		unsafe { (*self.pointer()).cqe }
	}
	
	#[inline(always)]
	fn completionEventCompleted(self) -> u32
	{
		unsafe { (*self.pointer()).comp_events_completed }
	}
	
	#[inline(always)]
	fn asynchronousEventCompleted(self) -> u32
	{
		unsafe { (*self.pointer()).async_events_completed }
	}
}

impl CompletionQueuePointer for *mut ibv_cq
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self
	}
}

impl CompletionQueuePointer for *mut ibv_cq_ex
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self.ibv_cq_ex_to_cq()
	}
}
