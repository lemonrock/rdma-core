// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CompletionQueueContext<'a, UnderlyingCompletionQueueContext: Sized>
where UnderlyingCompletionQueueContext: 'a
{
	type WorkCompletion: WorkCompletion;
	type PollIterator: Iterator<Item=Self::WorkCompletion>;
	
	#[inline(always)]
	fn isExtended(&self) -> bool;
	
	#[inline(always)]
	fn underlying(&mut self) -> &mut UnderlyingCompletionQueueContext;
	
	/// completionQueuePointerMaybeExtended can also be of type ibv_cq_ex
	#[inline(always)]
	fn destroy(&mut self, completionQueuePointerMaybeExtended: *mut ibv_cq);
	
	/// completionQueuePointerMaybeExtended can also be of type ibv_cq_ex
	#[inline(always)]
	fn iteratePoll(&'a mut self, completionQueuePointerMaybeExtended: *mut ibv_cq) -> Self::PollIterator;
}