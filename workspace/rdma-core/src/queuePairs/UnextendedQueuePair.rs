// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UnextendedQueuePair<'a>
{
	pointer: *mut ibv_qp,
	lifetime: &'a ProtectionDomain<'a>,
	multicastGroupMemberships: HashSet<MultiCastGroupIdentifier>,
}

impl<'a> Drop for UnextendedQueuePair<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		for multiCastGroupIdentifier in self.multicastGroupMemberships.drain()
		{
			panic_on_errno!(ibv_detach_mcast, self.pointer, &multiCastGroupIdentifier.multicastGroupId, multiCastGroupIdentifier.localIdentifier);
		}
		
		panic_on_errno!(ibv_destroy_qp, self.pointer);
	}
}

impl<'a> QueuePair<'a> for UnextendedQueuePair<'a>
{
	#[inline(always)]
	fn joinMultiCastGroup(&mut self, multiCastGroupIdentifier: MultiCastGroupIdentifier) -> bool
	{
		if likely(self.multicastGroupMemberships.insert(multiCastGroupIdentifier))
		{
			panic_on_errno!(ibv_attach_mcast, self.pointer, &multiCastGroupIdentifier.multicastGroupId, multiCastGroupIdentifier.localIdentifier);
			true
		}
		else
		{
			false
		}
	}
	
	#[inline(always)]
	fn leaveMultiCastGroup(&mut self, multiCastGroupIdentifier: MultiCastGroupIdentifier) -> bool
	{
		if likely(self.multicastGroupMemberships.remove(&multiCastGroupIdentifier))
		{
			panic_on_errno!(ibv_detach_mcast, self.pointer, &multiCastGroupIdentifier.multicastGroupId, multiCastGroupIdentifier.localIdentifier);
			true
		}
		else
		{
			false
		}
	}
}

impl<'a> UnextendedQueuePair<'a>
{
	#[inline(always)]
	fn new(&self, pointer: *mut ibv_qp, lifetime: &'a ProtectionDomain<'a>) -> UnextendedQueuePair<'a>
	{
		UnextendedQueuePair
		{
			pointer: pointer,
			lifetime: lifetime,
			multicastGroupMemberships: HashSet::with_capacity(1),
		}
	}
}
