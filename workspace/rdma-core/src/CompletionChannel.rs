// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct CompletionChannel<'a>
{
	pointer: *mut ibv_comp_channel,
	context: &'a Context,
	unextendedCompletionQueues: HashMap<usize, WithCompletionChannelUnextendedCompletionQueue>,
	extendedCompletionQueues: HashMap<usize, WithCompletionChannelExtendedCompletionQueue>,
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
			unextendedCompletionQueues: HashMap::new(),
			extendedCompletionQueues: HashMap::new(),
		}
	}
	
	#[inline(always)]
	pub fn fileDescriptorForEpoll(&self) -> FileDescriptor
	{
		unsafe { *self.pointer }.fd
	}
	
	#[inline(always)]
	pub fn createWithCompletionChannelUnextendedCompletionQueue(&'a mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> &'a WithCompletionChannelUnextendedCompletionQueue
	{
		let pointer = self.context.createUnextendedCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, self.pointer);
		let key = pointer as usize;
		self.unextendedCompletionQueues.insert(key, WithCompletionChannelUnextendedCompletionQueue::new(pointer));
		self.unextendedCompletionQueues.get(&key).unwrap()
	}
	
	#[inline(always)]
	pub fn createWithCompletionChannelExtendedCompletionQueue(&'a mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool) -> &'a WithCompletionChannelExtendedCompletionQueue
	{
		let pointer = self.context.createExtendedCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, workCompletionFlags, lockLessButNotThreadSafe, self.pointer);
		let key = pointer as usize;
		self.extendedCompletionQueues.insert(key, WithCompletionChannelExtendedCompletionQueue::new(pointer));
		self.extendedCompletionQueues.get(&key).unwrap()
	}
	
	#[inline(always)]
	pub fn waitForCompletionEvent<R, Unextended: Fn(&WithCompletionChannelUnextendedCompletionQueue, *mut c_void) -> R, Extended: Fn(&WithCompletionChannelExtendedCompletionQueue, *mut c_void) -> R>(&self, handleUnextendedEventAndContext: Unextended, handleExtendedEventAndContext: Extended) -> R
	{
		let mut cq = null_mut();
		let mut context = null_mut();
		panic_on_error!(ibv_get_cq_event, self.pointer, &mut cq, &mut context);
		let key = cq as usize;
		if let Some(unextendedCompletionQueue) = self.unextendedCompletionQueues.get(&key)
		{
			handleUnextendedEventAndContext(unextendedCompletionQueue, context)
		}
		else if let Some(extendedCompletionQueue) = self.extendedCompletionQueues.get(&key)
		{
			handleExtendedEventAndContext(extendedCompletionQueue, context)
		}
		else
		{
			panic!("Unknown completion queue");
		}
	}
}

