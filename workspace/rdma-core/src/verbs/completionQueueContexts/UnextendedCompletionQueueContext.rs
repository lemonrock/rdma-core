// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UnextendedCompletionQueueContext<UnderlyingCompletionQueueContext: Sized>
{
	underlying: UnderlyingCompletionQueueContext,
}

impl<UnderlyingCompletionQueueContext> CompletionQueueContext<UnderlyingCompletionQueueContext> for UnextendedCompletionQueueContext<UnderlyingCompletionQueueContext>
{
	#[inline(always)]
	fn isExtended(&self) -> bool
	{
		false
	}
	
	#[inline(always)]
	fn underlying(&mut self) -> &mut UnderlyingCompletionQueueContext
	{
		&mut self.underlying
	}
	
	#[inline(always)]
	fn destroy(&mut self, mut completionQueuePointerMaybeExtended: *mut ibv_cq)
	{
		completionQueuePointerMaybeExtended.destroy();
	}
}

pub const UnextendedCompletionQueuePollArraySize: usize = 32;

impl<UnderlyingCompletionQueueContext> UnextendedCompletionQueueContext<UnderlyingCompletionQueueContext>
{
	/// Returns number of additional work completions added; it is recommended that `into` is empty
	#[inline(always)]
	pub fn poll(completionQueuePointer: *mut ibv_cq, into: &mut ArrayVec<[UnextendedWorkCompletion; UnextendedCompletionQueuePollArraySize]>) -> usize
	{
		let length = into.len();
		
		let space = UnextendedCompletionQueuePollArraySize - length;
		
		if unlikely(space == 0)
		{
			return 0;
		}
		
		let fillFrom = unsafe { transmute(into.as_mut_ptr().offset(length as isize)) };
		
		let result = completionQueuePointer.ibv_poll_cq(space as i32, fillFrom);
		if likely(result >= 0)
		{
			debug_assert!(result as usize <= space, "Overfilled; defect in ibv_poll_cq()");
			let increasedBy = result as usize;
			unsafe { into.set_len(length + increasedBy) };
			increasedBy
		}
		else
		{
			let errno = errno();
			panic!("rust_ibv_poll_cq failed with result '{}' error number '{}' ('{}')", result, errno.0, errno);
		}
	}
}
