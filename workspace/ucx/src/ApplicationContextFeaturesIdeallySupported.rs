// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApplicationContextFeaturesIdeallySupported
{
	pub remoteMemoryAccess: bool,
	pub atomicOperations32BitsWide: bool,
	pub atomicOperations64BitsWide: bool,
	pub interruptNotificationIeWakeUp: bool,
}

impl Default for ApplicationContextFeaturesIdeallySupported
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			remoteMemoryAccess: true,
			atomicOperations32BitsWide: true,
			atomicOperations64BitsWide: true,
			interruptNotificationIeWakeUp: false,
		}
	}
}

impl ApplicationContextFeaturesIdeallySupported
{
	#[inline(always)]
	fn as_u64(&self, tagSenderMask: u64) -> u64
	{
		use ucp_feature::*;
		
		let mut features = 0;
		
		if likely(tagSenderMask != 0)
		{
			features |= UCP_FEATURE_TAG as u64
		}
		
		if likely(self.remoteMemoryAccess)
		{
			features |= UCP_FEATURE_RMA as u64
		}
		
		if likely(self.atomicOperations32BitsWide)
		{
			features |= UCP_FEATURE_AMO32 as u64
		}
		
		if likely(self.atomicOperations64BitsWide)
		{
			features |= UCP_FEATURE_AMO64 as u64
		}
		
		if unlikely(self.interruptNotificationIeWakeUp)
		{
			features |= UCP_FEATURE_WAKEUP as u64
		}
		
		features
	}
}
