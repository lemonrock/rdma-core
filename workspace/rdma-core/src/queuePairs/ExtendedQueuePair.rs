// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ExtendedQueuePair<'a>
{
	unextendedQueuePair: UnextendedQueuePair<'a>,
	extendedReliableConnectionDomain: &'a ExtendedReliableConnectionDomain<'a>,
}

impl<'a> QueuePair<'a> for ExtendedQueuePair<'a>
{
	#[inline(always)]
	fn joinMultiCastGroup(&mut self, multiCastGroupIdentifier: MultiCastGroupIdentifier) -> bool
	{
		self.unextendedQueuePair.joinMultiCastGroup(multiCastGroupIdentifier)
	}
	
	#[inline(always)]
	fn leaveMultiCastGroup(&mut self, multiCastGroupIdentifier: MultiCastGroupIdentifier) -> bool
	{
		self.unextendedQueuePair.leaveMultiCastGroup(multiCastGroupIdentifier)
	}
	
	#[inline(always)]
	fn modify(&self, attributeMask: AttributeFlags::Flags, attributes: &mut ibv_qp_attr)
	{
		self.unextendedQueuePair.modify(attributeMask, attributes)
	}
	
	#[inline(always)]
	fn attributes(&self) -> (ibv_qp_attr, ibv_qp_init_attr)
	{
		self.unextendedQueuePair.attributes()
	}
}
