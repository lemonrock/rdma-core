// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct EventChannelVerbMapEntry<'a>
{
	protectionDomain: *mut ibv_pd,
	completionChannel: Box<EPollContextChoice<'a>>,
}

impl<'a> Drop for EventChannelVerbMapEntry<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.protectionDomain.destroy();
	}
}

impl<'a> VerbMapEntry<'a> for EventChannelVerbMapEntry<'a>
{
	type ConstructionParameters = EPoll<EPollContextChoice<'a>>;
	
	#[inline(always)]
	fn create(constructionParameters: &'a Self::ConstructionParameters, verbs: *mut ibv_context) -> Self
	{
		Self
		{
			protectionDomain: verbs.allocateProtectionDomain(),
			completionChannel: CompletionChannel::new(constructionParameters, verbs)
		}
	}
}

impl<'a> EventChannelVerbMapEntry<'a>
{
	#[inline(always)]
	pub fn createCompletionQueue(&mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void)
	{
		match *self.completionChannel
		{
			EPollContextChoice::CompletionChannel(ref mut context) => context.createCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext),
			_ => (),
		}
	}
	
	#[inline(always)]
	pub fn createExtendedCompletionQueue(&mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool)
	{
		match *self.completionChannel
		{
			EPollContextChoice::CompletionChannel(ref mut context) => context.createExtendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, workCompletionFlags, lockLessButNotThreadSafe),
			_ => (),
		}
	}
}
