// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ExtendedCompletionQueueContext<UnderlyingCompletionQueueContext: CompletionQueueContext>
{
	underlying: UnderlyingCompletionQueueContext,
	isCurrentlyBeingPolled: bool,
}

impl<UnderlyingCompletionQueueContext: CompletionQueueContext> CompletionQueueContext for ExtendedCompletionQueueContext<UnderlyingCompletionQueueContext>
{
	#[inline(always)]
	fn isExtended(&self) -> bool
	{
		true
	}
}

impl<UnderlyingCompletionQueueContext: CompletionQueueContext> ExtendedCompletionQueueContext<UnderlyingCompletionQueueContext>
{
	#[inline(always)]
	pub fn destroy(&mut self, completionQueuePointer: *mut ibv_cq_ex)
	{
		if self.isCurrentlyBeingPolled
		{
			self.endPolling(completionQueuePointer);
		}
	}
	
	#[inline(always)]
	pub fn underlying(&mut self) -> &mut UnderlyingCompletionQueueContext
	{
		&mut self.underlying
	}
	
	/// NOTE WELL: Once poll() is called, the previous item will be invalid
	#[inline(always)]
	pub fn poll(&mut self, completionQueuePointer: *mut ibv_cq_ex) -> Option<ExtendedWorkCompletion>
	{
		if likely(self.isCurrentlyBeingPolled)
		{
			let result = completionQueuePointer.ibv_next_poll();
			debug_assert!(result >= 0, "result was negative '{}'", result);
			if likely(result == 0)
			{
				Some(ExtendedWorkCompletion(completionQueuePointer))
			}
			else
			{
				self.endPolling(completionQueuePointer);
				self.isCurrentlyBeingPolled = false;
				
				if likely(result == E::ENOENT)
				{
					None
				}
				else
				{
					panic!("ibv_next_poll() returned an error number '{}'", result);
				}
			}
		}
		else
		{
			let mut attributes = ibv_poll_cq_attr
			{
				comp_mask: 0
			};
			
			let result = completionQueuePointer.ibv_start_poll(&mut attributes);
			debug_assert!(result >= 0, "result was negative '{}'", result);
			if likely(result == 0)
			{
				self.isCurrentlyBeingPolled = false;
				Some(ExtendedWorkCompletion(completionQueuePointer))
			}
			else
			{
				// NOTE: We MUST NOT call self.endPolling() here
				
				if likely(result == E::ENOENT)
				{
					None
				}
				else
				{
					panic!("ibv_start_poll() returned an error number '{}'", result);
				}
			}
		}
	}
	
	#[inline(always)]
	fn endPolling(&mut self, completionQueuePointer: *mut ibv_cq_ex)
	{
		completionQueuePointer.ibv_end_poll()
	}
}
