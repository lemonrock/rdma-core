// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct CompletionChannel<'a>
{
	ePoll: &'a EPoll<EPollContextChoice<'a>>,
	completionChannel: *mut ibv_comp_channel,
	completionQueues: Vec<*mut ibv_cq>,
}

impl<'a> Drop for CompletionChannel<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		for completionQueue in self.completionQueues.drain(..)
		{
			let mut context = completionQueue.getBoxPointerContext::<CompletionQueueContextChoice>();
			context.destroy(completionQueue);
			drop(context);
		}
		
		let fileDescriptor = self.fileDescriptorForEPoll();
		self.ePoll.deregister(fileDescriptor);
		self.completionChannel.destroy();
	}
}

impl<'a> EPollContext for CompletionChannel<'a>
{
	#[inline(always)]
	fn fileDescriptorForEPoll(&self) -> RawFd
	{
		self.completionChannel.fileDescriptorForEPoll()
	}
	
	#[inline(always)]
	fn processEvents(&mut self)
	{
		let maximumCompletionQueues = self.completionQueues.len();
		
		debug_assert!(maximumCompletionQueues != 0, "processEvents called but no registered completion queues - what is epoll or RDMA CM playing at?");
		
		let mut counters = HashMap::with_capacity(maximumCompletionQueues);
		
		loop
		{
			let mut completionQueue = unsafe { uninitialized() };
			let mut completionQueueContext = unsafe { uninitialized() };
			
			let result = unsafe { ibv_get_cq_event(self.completionChannel, &mut completionQueue, &mut completionQueueContext) };
			debug_assert!(result == 0 || result == -1, "ibv_get_cq_event returned a result '{}' which was not 0 or -1", result);
			
			if unlikely(result == -1)
			{
				debug_assert!(completionQueue.is_null(), "completionQueue was not null");
				debug_assert!(completionQueueContext.is_null(), "completionQueueContext was not null");
				
				let errno = errno();
				if likely(errno.0 == E::EAGAIN)
				{
					break;
				}
				panic!("ibv_get_cq_event failed with error number '{}' ('{}')", errno.0, errno);
			}
			else
			{
				debug_assert!(!completionQueue.is_null(), "completionQueue was null - this should not occur");
				debug_assert!(!completionQueueContext.is_null(), "completionQueueContext was null - this should not occur");
				
				let mut context = unsafe { Box::from_raw(completionQueueContext as *mut CompletionQueueContextChoice) };
				context.pollToExhaustion(completionQueue, |unextendedWorkCompletion| {}, |extendedWorkCompletion| {});
				forget(context);
				
				*counters.entry(completionQueue).or_insert(0) += 1;
			}
		}
		
		for (eventQueue, counter) in counters.drain()
		{
			unsafe { ibv_ack_cq_events(eventQueue, counter) };
		}
	}
}

impl<'a> CompletionChannel<'a>
{
	// Mellanox calls these "completion EQs", but I (RJC) don't know what that stands for. ?Event Queues?
	const DistributeToLeastLoadedInternalQueue: u32 = 0;
	
	#[inline(always)]
	pub fn new(ePoll: &'a EPoll<EPollContextChoice<'a>>, verbs: *mut ibv_context) -> Box<EPollContextChoice>
	{
		let completionChannel = verbs.createCompletionChannel();
		
		let this = Self
		{
			ePoll: ePoll,
			completionChannel: completionChannel,
			completionQueues: Vec::with_capacity(32),
		};
		
		ePoll.registerEdgeTriggeredIn(EPollContextChoice::CompletionChannel(this))
	}
	
	#[inline(always)]
	pub fn createCompletionQueue(&mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void)
	{
		let verbs = self.completionChannel.verbs();
		let completionQueue = verbs.createUnextendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, Self::DistributeToLeastLoadedInternalQueue, self.completionChannel);
		self.completionQueues.push(completionQueue);
	}
	
	#[inline(always)]
	pub fn createExtendedCompletionQueue(&mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool)
	{
		let verbs = self.completionChannel.verbs();
		let extendedCompletionQueue = verbs.createExtendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, Self::DistributeToLeastLoadedInternalQueue, workCompletionFlags, lockLessButNotThreadSafe, self.completionChannel);
		self.completionQueues.push(extendedCompletionQueue.pointer());
	}
}
