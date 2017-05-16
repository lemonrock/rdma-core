// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(dead_code)]
pub struct ExtendedSharedReceiveQueue<'a, C: CompletionQueue>
where C: 'a
{
	pub(crate) unextendedSharedReceiveQueue: UnextendedSharedReceiveQueue<'a>,
	pub(crate) extendedReliableConnectionDomain: &'a ExtendedReliableConnectionDomain<'a>,
	pub(crate) completionQueue: &'a C,
}

impl<'a, C: CompletionQueue> SharedReceiveQueue for ExtendedSharedReceiveQueue<'a, C>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_srq
	{
		self.unextendedSharedReceiveQueue.pointer()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn settings(&mut self) -> &mut SharedReceiveQueueSettings
	{
		self.unextendedSharedReceiveQueue.settings()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn context(&self) -> &Context
	{
		self.unextendedSharedReceiveQueue.context()
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn isValidForProtectionDomain<'b>(&self, protectionDomain: &ProtectionDomain<'b>) -> bool
	{
		self.unextendedSharedReceiveQueue.isValidForProtectionDomain(protectionDomain)
	}
}
