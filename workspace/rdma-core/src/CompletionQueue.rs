// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct CompletionQueue<'a>
{
	pointer: *mut ibv_cq,
	lifetime: Option<&'a CompletionChannel<'a>>,
}

impl<'a> Drop for CompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_cq, self.pointer);
	}
}

impl<'a> CompletionQueue<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_cq, lifetime: Option<&'a CompletionChannel>) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
		}
	}
	
	#[inline(always)]
	pub fn resize(&self, atLeastThisNumberOfCompletionQueueEvents: u31)
	{
		panic_on_error!(ibv_resize_cq, self.pointer, atLeastThisNumberOfCompletionQueueEvents as i32);
	}
	
//	/// See also <https://linux.die.net/man/3/ibv_get_cq_event>
//	#[inline(always)]
//	pub fn getEvent(&self) -> CompletionQueue<'a>
//	{
//		let channelPointer = match self.lifetime
//		{
//			None => null_mut(),
//			Some(channel) => channel.pointer,
//		};
//
//		panic_on_error!(ibv_get_cq_event, channelPointer, cq, context);
//	}
	
	/*
		Missing: ibv_req_notify_cq
	
		pub fn ibv_get_cq_event(channel: *mut ibv_comp_channel, cq: *mut *mut ibv_cq, cq_context: *mut *mut c_void) -> c_int;
	*/
	
	/// This is relatively expensive to perform
	#[inline(always)]
	pub fn acknowledgeEvents(&self, numberOfEventsToAcknowledge: u32)
	{
		unsafe { ibv_ack_cq_events(self.pointer, numberOfEventsToAcknowledge) }
	}
}
