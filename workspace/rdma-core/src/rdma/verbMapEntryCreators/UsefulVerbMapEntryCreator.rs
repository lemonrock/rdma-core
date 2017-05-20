// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UsefulVerbMapEntryCreator
{
	protectionDomain: *mut ibv_pd,
	completionChannel: *mut ibv_comp_channel,
}

impl Drop for UsefulVerbMapEntryCreator
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.completionChannel.destroy();
		self.protectionDomain.destroy();
	}
}

impl VerbMapEntryCreator for UsefulVerbMapEntryCreator
{
	type Entry = Self;
	
	#[inline(always)]
	fn create(verbs: *mut ibv_context) -> Self::Entry
	{
		let protectionDomain = verbs.allocateProtectionDomain();
		let completionChannel = verbs.createCompletionChannel();
		//const DistributeToLeastLoadedInternalQueue: u32 = 0; // Mellanox calls these "completion EQs", but I don't know what that stands for
		//let completionQueue = verbs.createUnextendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, DistributeToLeastLoadedInternalQueue, completionChannel);
		
		Self
		{
			protectionDomain: protectionDomain,
			completionChannel: completionChannel,
			// Do we want one of these per core?
		}
	}
}

impl UsefulVerbMapEntryCreator
{
	#[inline(always)]
	pub(crate) fn protectionDomain(&self) -> *mut ibv_pd
	{
		self.protectionDomain
	}
}
