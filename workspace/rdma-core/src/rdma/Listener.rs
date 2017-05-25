// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct Listener
{
	context: Box<CommunicationIdentifierContext>,
	addressing: Addressing,
	backLog: u32
}

impl Listener
{
	#[inline(always)]
	pub fn new(context: Box<CommunicationIdentifierContext>, addressing: Addressing, backLog: u32) -> Self
	{
		Self
		{
			context: context,
			addressing: addressing,
			backLog: backLog,
		}
	}
	
	#[inline(always)]
	fn to_rdma_cm_id(self, eventChannel: *mut rdma_event_channel) -> *mut rdma_cm_id
	{
		debug_assert!(!eventChannel.is_null(), "eventChannel is null");
		eventChannel.newListeningCommunicationIdentifier(self.context, self.addressing, self.backLog)
	}
}
