// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub enum CompletionQueueContextChoice
{
	Unextended(UnextendedCompletionQueueContext),
	Extended(ExtendedCompletionQueueContext),
}

impl CompletionQueueContextChoice
{
	/// completionQueuePointerMaybeExtended can also be of type ibv_cq_ex
	#[inline(always)]
	pub fn destroy(&mut self, completionQueuePointerMaybeExtended: *mut ibv_cq)
	{
		match *self
		{
			CompletionQueueContextChoice::Unextended(ref mut context) => context.destroy(completionQueuePointerMaybeExtended),
			CompletionQueueContextChoice::Extended(ref mut context) => context.destroy(completionQueuePointerMaybeExtended),
		}
	}
	
	#[inline(always)]
	pub fn pollToExhaustion<UnextendedWorkCompletionUser: Fn(UnextendedWorkCompletion), ExtendedWorkCompletionUser: Fn(ExtendedWorkCompletion)>(&mut self, completionQueuePointerMaybeExtended: *mut ibv_cq, unextendedWorkCompletionUser: UnextendedWorkCompletionUser, extendedWorkCompletionUser: ExtendedWorkCompletionUser)
	{
		match *self
		{
			CompletionQueueContextChoice::Unextended(ref mut context) =>
			{
				context.pollToExhaustion(completionQueuePointerMaybeExtended, unextendedWorkCompletionUser)
			},
			CompletionQueueContextChoice::Extended(ref mut context) =>
			{
				context.pollToExhaustion(completionQueuePointerMaybeExtended, extendedWorkCompletionUser)
			},
		}
	}
}
