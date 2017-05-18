// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UnreliableDatagramEventData(pub(crate) *mut rdma_cm_event);

impl EventData for UnreliableDatagramEventData
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut rdma_cm_event
	{
		self.0
	}
	
	/// .len() may be greater than the amount of data sent by the remote side, but all excess bytes are g'teed to be zero
	#[inline(always)]
	fn privateData<'a>(&'a self) -> Option<&'a [u8]>
	{
		privateDataUd(self.data())
	}
	
	#[inline(always)]
	fn remoteQueuePairNumber(&self) -> QueuePairNumber
	{
		remoteQueuePairNumberUd(self.data())
	}
}

impl UnreliableDatagramEventData
{
	#[inline(always)]
	fn data(&self) -> rdma_ud_param
	{
		unsafe { (*self.pointer()).param.ud }
	}
	
	#[inline(always)]
	pub fn addressHandleAttributesClone(&mut self) -> ibv_ah_attr
	{
		self.data().ah_attr.clone()
	}
	
	#[inline(always)]
	pub fn qKey(&self) -> u32
	{
		self.data().qkey
	}
}
