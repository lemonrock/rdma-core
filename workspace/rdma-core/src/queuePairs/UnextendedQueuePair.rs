// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


// extendedReliableConnectionDomain: &'a ExtendedReliableConnectionDomain<'a>,

pub struct UnextendedQueuePair<'a, SendC: CompletionQueue, ReceiveC: CompletionQueue, SharedReceive: SharedReceiveQueue>
where SendC: 'a, ReceiveC: 'a, SharedReceive: 'a
{
	pointer: *mut ibv_qp,
	capabilities: ibv_qp_cap,
	multicastGroupMemberships: HashSet<MultiCastGroupIdentifier>,
	lifetime: (&'a ProtectionDomain<'a>, &'a SendC, &'a ReceiveC, Option<&'a SharedReceive>),
}

impl<'a, SendC: CompletionQueue, ReceiveC: CompletionQueue, SharedReceive: SharedReceiveQueue> Drop for UnextendedQueuePair<'a, SendC, ReceiveC, SharedReceive>
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

impl<'a, SendC: CompletionQueue, ReceiveC: CompletionQueue, SharedReceive: SharedReceiveQueue> QueuePair for UnextendedQueuePair<'a, SendC, ReceiveC, SharedReceive>
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
	
	#[inline(always)]
	fn modify(&self, attributeMask: AttributeFlags::Flags, attributes: &mut ibv_qp_attr)
	{
		panic_on_error!(ibv_modify_qp, self.pointer, attributes, attributeMask.bits());
	}
	
	#[inline(always)]
	fn attributes(&self) -> (ibv_qp_attr, ibv_qp_init_attr)
	{
		let mut attributes = unsafe { zeroed() };
		let mut initAttributes = unsafe { zeroed() };
		panic_on_error!(ibv_query_qp, self.pointer, &mut attributes, AttributeFlags::All.bits(), &mut initAttributes);
		(attributes, initAttributes)
	}
	
	#[inline(always)]
	fn capabilities(&self) -> &ibv_qp_cap
	{
		&self.capabilities
	}
}

impl<'a, SendC: CompletionQueue, ReceiveC: CompletionQueue, SharedReceive: SharedReceiveQueue> UnextendedQueuePair<'a, SendC, ReceiveC, SharedReceive>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut ibv_qp, capabilities: ibv_qp_cap, lifetime: (&'a ProtectionDomain<'a>, &'a SendC, &'a ReceiveC, Option<&'a SharedReceive>)) -> Self
	{
		UnextendedQueuePair
		{
			pointer: pointer,
			capabilities: capabilities,
			multicastGroupMemberships: HashSet::with_capacity(1),
			lifetime: lifetime,
		}
	}
}
