// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


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
	pub fn createCompletionQueue(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> CompletionQueue<'a>
	{
		self.context.createCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, Some(self))
	}
	
	/*
		pub fn ibv_create_cq(context: *mut ibv_context, cqe: c_int, cq_context: *mut c_void, channel: *mut ibv_comp_channel, comp_vector: c_int) -> *mut ibv_cq;
	*/
}
