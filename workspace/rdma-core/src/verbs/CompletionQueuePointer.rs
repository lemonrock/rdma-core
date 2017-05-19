// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CompletionQueuePointer: HasContextPointer
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq;
	
	#[inline(always)]
	fn completionChannel(self) -> *mut ibv_comp_channel
	{
		unsafe { (*self.pointer()).channel }
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
	
	/// ibv_poll_cq - Poll a CQ for work completions
	/// @cq:the CQ being polled
	/// @num_entries:maximum number of completions to return
	/// @wc:array of at least @num_entries of &struct ibv_wc where completions
	///   will be returned
	///
	/// Poll a CQ for (possibly multiple) completions.  If the return value
	/// is < 0, an error occurred.  If the return value is >= 0, it is the
	/// number of completions returned.  If the return value is
	/// non-negative and strictly less than num_entries, then the CQ was
	/// emptied.
	#[inline(always)]
	fn ibv_poll_cq(self, num_entries: c_int, wc: *mut ibv_wc) -> c_int
	{
		unsafe { self.verbs().ops().poll_cq.unwrap()(self.pointer(), num_entries, wc) }
	}

	/// ibv_req_notify_cq - Request completion notification on a CQ.  An
	///   event will be added to the completion channel associated with the
	///   CQ when an entry is added to the CQ.
	/// @cq: The completion queue to request notification for.
	/// @solicited_only: If non-zero, an event will be generated only for
	///   the next solicited CQ entry.  If zero, any CQ entry, solicited or
	///   not, will generate an event.
	#[inline(always)]
	fn ibv_req_notify_cq(self, solicited_only: bool) -> c_int
	{
		unsafe
		{
			self.verbs().ops().req_notify_cq.unwrap()(self.pointer(), if solicited_only
			{
				1
			}
			else
			{
				0
			})
		}
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
