// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub trait CompletionQueue<'a>: Drop
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq;
	
	#[inline(always)]
	fn resize(&self, atLeastThisNumberOfCompletionQueueEvents: u31)
	{
		panic_on_error!(ibv_resize_cq, self.pointer(), atLeastThisNumberOfCompletionQueueEvents as i32);
	}
	
	/// This is relatively expensive to perform
	#[inline(always)]
	fn acknowledgeEvents(&self, numberOfEventsToAcknowledge: u32)
	{
		unsafe { ibv_ack_cq_events(self.pointer(), numberOfEventsToAcknowledge) }
	}
	
	#[inline(always)]
	fn requestCompletionNotificationsForSolicitedAndErrorEventsOnly(&self)
	{
		panic_on_errno!(rust_ibv_req_notify_cq, self.pointer(), 1)
	}
	
	#[inline(always)]
	fn requestCompletionNotificationsForAllEvents(&self)
	{
		panic_on_errno!(rust_ibv_req_notify_cq, self.pointer(), 0)
	}
	
//	/// See also <https://linux.die.net/man/3/ibv_get_cq_event>
//	#[inline(always)]
//	fn getEvent(&self) -> CompletionQueue<'a>
//	{
//		let channelPointer = match self.lifetime
//		{
//			None => null_mut(),
//			Some(channel) => channel.pointer,
//		};
//
//		panic_on_error!(ibv_get_cq_event, channelPointer, cq, context);
//	}
}