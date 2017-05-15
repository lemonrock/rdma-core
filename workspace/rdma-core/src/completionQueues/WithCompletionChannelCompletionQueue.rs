// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait WithCompletionChannelCompletionQueue: CompletionQueue
{
	/// This is relatively expensive to perform
	#[inline(always)]
	fn acknowledgeEvents(&self, numberOfEventsToAcknowledge: u32)
	{
		unsafe { ibv_ack_cq_events(self.pointer(), numberOfEventsToAcknowledge) }
	}
	
	#[inline(always)]
	fn requestOneShotCompletionNotificationsForSolicitedAndErrorEventsOnly(&self)
	{
		panic_on_errno!(rust_ibv_req_notify_cq, self.pointer(), 1)
	}
	
	#[inline(always)]
	fn requestOneShotCompletionNotificationsForAllEvents(&self)
	{
		panic_on_errno!(rust_ibv_req_notify_cq, self.pointer(), 0)
	}
}
