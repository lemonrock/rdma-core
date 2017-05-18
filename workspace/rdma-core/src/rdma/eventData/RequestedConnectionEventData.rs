// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct RequestedConnectionEventData(pub(crate) *mut rdma_cm_event);

impl EventData for RequestedConnectionEventData
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut rdma_cm_event
	{
		self.0
	}
	
	#[inline(always)]
	fn privateData<'a>(&'a self) -> Option<&'a [u8]>
	{
		privateDataConn(self.data())
	}
	
	#[inline(always)]
	fn remoteQueuePairNumber(&self) -> QueuePairNumber
	{
		remoteQueuePairNumberConn(self.data())
	}
}

impl ConnectionEventData for RequestedConnectionEventData
{
}

impl RequestedConnectionEventData
{
	#[inline(always)]
	pub fn retrySendOperationsCount(&self) -> u8
	{
		self.data().retry_count
	}
}
