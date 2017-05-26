// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ExtendedCompletionQueueContext
{
	isCurrentlyBeingPolled: bool,
}

impl CompletionQueueContext for ExtendedCompletionQueueContext
{
	type WorkCompletion = ExtendedWorkCompletion;
	
	#[inline(always)]
	fn new() -> Self
	{
		Self
		{
			isCurrentlyBeingPolled: false
		}
	}
	
	#[inline(always)]
	fn destroy(&mut self, mut completionQueuePointerMaybeExtended: *mut ibv_cq)
	{
		if self.isCurrentlyBeingPolled
		{
			self.endPolling((completionQueuePointerMaybeExtended as *mut ibv_cq_ex));
			self.isCurrentlyBeingPolled = false;
		}
		
		completionQueuePointerMaybeExtended.destroy();
	}
	
	#[inline(always)]
	fn pollToExhaustion<WorkCompletionUser: Fn(Self::WorkCompletion)>(&mut self, completionQueuePointerMaybeExtended: *mut ibv_cq, workCompletionUser: WorkCompletionUser)
	{
		let extendedCompletionQueuePointer = completionQueuePointerMaybeExtended as *mut ibv_cq_ex;
		
		loop
		{
			if likely(self.isCurrentlyBeingPolled)
			{
				let result = extendedCompletionQueuePointer.ibv_next_poll();
				debug_assert!(result >= 0, "result was negative '{}'", result);
				if likely(result == 0)
				{
					workCompletionUser(ExtendedWorkCompletion(extendedCompletionQueuePointer))
				}
				else
				{
					self.endPolling(extendedCompletionQueuePointer);
					self.isCurrentlyBeingPolled = false;
					
					if likely(result == E::ENOENT)
					{
						break;
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
				
				let result = extendedCompletionQueuePointer.ibv_start_poll(&mut attributes);
				debug_assert!(result >= 0, "result was negative '{}'", result);
				if likely(result == 0)
				{
					self.isCurrentlyBeingPolled = false;
					workCompletionUser(ExtendedWorkCompletion(extendedCompletionQueuePointer));
				}
				else
				{
					// NOTE: We MUST NOT call self.endPolling() here
					
					if likely(result == E::ENOENT)
					{
						break;
					}
					else
					{
						panic!("ibv_start_poll() returned an error number '{}'", result);
					}
				}
			}
		}
	}
}

impl ExtendedCompletionQueueContext
{
	#[inline(always)]
	fn endPolling(&mut self, extendedCompletionQueuePointer: *mut ibv_cq_ex)
	{
		extendedCompletionQueuePointer.ibv_end_poll()
	}
}
