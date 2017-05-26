// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UnextendedCompletionQueueContext;

const UnextendedCompletionQueuePollArraySize: usize = 32;

impl CompletionQueueContext for UnextendedCompletionQueueContext
{
	type WorkCompletion = UnextendedWorkCompletion;
	
	#[inline(always)]
	fn new() -> Self
	{
		UnextendedCompletionQueueContext
	}
	
	#[inline(always)]
	fn destroy(&mut self, mut completionQueuePointerMaybeExtended: *mut ibv_cq)
	{
		completionQueuePointerMaybeExtended.destroy();
	}
	
	#[inline(always)]
	fn pollToExhaustion<WorkCompletionUser: Fn(Self::WorkCompletion)>(&mut self, completionQueuePointerMaybeExtended: *mut ibv_cq, workCompletionUser: WorkCompletionUser)
	{
		let mut into: [Self::WorkCompletion; UnextendedCompletionQueuePollArraySize] = unsafe { uninitialized() };
		
		loop
		{
			let result = completionQueuePointerMaybeExtended.ibv_poll_cq(UnextendedCompletionQueuePollArraySize as i32, into.as_mut_ptr() as *mut _ as *mut _);
			if unlikely(result < 0)
			{
				let errno = errno();
				panic!("rust_ibv_poll_cq failed with result '{}' error number '{}' ('{}')", result, errno.0, errno);
			}
			else
			{
				let size = result as usize;
				debug_assert!(size <= UnextendedCompletionQueuePollArraySize, "Overfilled; defect in ibv_poll_cq()");
				let mut index = 0;
				while index < size
				{
					let workCompletion = unsafe { replace(into.get_unchecked_mut(index), uninitialized()) };
					workCompletionUser(workCompletion);
					index += 1;
				}
				
				if likely(size != UnextendedCompletionQueuePollArraySize)
				{
					break;
				}
			}
		}
		
		forget(into);
	}
}
