// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct CompletionChannel<'a>
{
	pointer: *mut ibv_comp_channel,
	context: &'a Context,
}

impl<'a> Drop for CompletionChannel<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_comp_channel, self.pointer);
	}
}

impl<'a> CompletionChannel<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_comp_channel, context: &'a Context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			context: context,
		}
	}
	
	#[inline(always)]
	pub fn createWithCompletionChannelUnextendedCompletionQueue(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> WithCompletionChannelUnextendedCompletionQueue<'a>
	{
		let pointer = self.context.createUnextendedCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, self.pointer);
		WithCompletionChannelUnextendedCompletionQueue::new(pointer, self)
	}
	
	#[inline(always)]
	pub fn createWithCompletionChannelExtendedCompletionQueue(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool) -> WithCompletionChannelExtendedCompletionQueue<'a>
	{
		let pointer = self.context.createExtendedCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, workCompletionFlags, lockLessButNotThreadSafe, self.pointer);
		WithCompletionChannelExtendedCompletionQueue::new(pointer, self)
	}
}

//	See https://www.mankier.com/3/ibv_get_cq_event
//
//	#[inline(always)]
//	fn getEvent(&self) -> Option<CompletionQueueEvent<'a>>
//	{
//		For the 'global' completion channel, do we pass null_mut() instead of self.pointer?
//
//		panic_on_error!(ibv_get_cq_event, self.pointer, cq, context);
// Now ack with  ibv_ack_cq_events() - expensive...
// Should we 'own' pointers?
//	}
//
//	pub struct CompletionQueueEvent<'a, C: CompletionQueue<'a>>
//	where C: 'a
//	{
//		completionQueue: C,
//		context: *mut c_void,
//	}
//
//	The completion channel has a file descriptor which one can use with epoll() for non-blocking polling of events
//
