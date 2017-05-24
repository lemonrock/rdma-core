// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_contextEx: Sized
{
	#[inline(always)]
	fn destroy(self);
	
	#[inline(always)]
	fn attributes(self) -> ibv_device_attr;
	
	#[inline(always)]
	fn extendedAttributes(self) -> ibv_device_attr_ex;
	
	#[inline(always)]
	fn queryRealTimeRawClock(self) -> timespec;
	
	#[doc(hidden)]
	#[inline(always)]
	fn data(self) -> ibv_context;
	
	#[doc(hidden)]
	#[inline(always)]
	fn ops(self) -> ibv_context_ops
	{
		self.data().ops
	}
	
	#[inline(always)]
	fn numberOfCompletionVectors(self) -> u32;
	
	#[inline(always)]
	fn commandRawFd(self) -> RawFd;
	
	#[inline(always)]
	fn asyncRawFd(self) -> RawFd;
	
	#[inline(always)]
	fn blockOnAsynchronousEvent(self) -> ibv_async_event;
	
	#[inline(always)]
	fn allocateProtectionDomain(self) -> *mut ibv_pd;
	
	#[inline(always)]
	fn createCompletionChannel(self) -> *mut ibv_comp_channel;
	
	#[inline(always)]
	fn createUnextendedCompletionQueueWithoutCompletionChannel(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> *mut ibv_cq;
	
	#[inline(always)]
	fn createUnextendedCompletionQueue(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, potentiallyNullCompletionChannel: *mut ibv_comp_channel) -> *mut ibv_cq;
	
	#[inline(always)]
	fn createExtendedCompletionQueueWithoutCompletionChannel(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool) -> *mut ibv_cq_ex;
	
	#[inline(always)]
	fn createExtendedCompletionQueue(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool, potentiallyNullCompletionChannel: *mut ibv_comp_channel) -> *mut ibv_cq_ex;
}

impl ibv_contextEx for *mut ibv_context
{
	#[inline(always)]
	fn destroy(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let result = unsafe { ibv_close_device(self) };
		debug_assert!(result == 0 || result == 1, "ibv_close_device returned a result '{}' which was not 0 or 1", result);
		if unlikely(result == -1)
		{
			// Examination of C source code suggests it can't actually fail
			panic!("ibv_close_device failed (no further details)");
		}
	}
	
	#[inline(always)]
	fn attributes(self) -> ibv_device_attr
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let mut attributes = unsafe { zeroed() };
		panic_on_error!(ibv_query_device, self, &mut attributes);
		attributes
	}
	
	#[inline(always)]
	fn extendedAttributes(self) -> ibv_device_attr_ex
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let input = unsafe { zeroed() };
		let mut attributes = unsafe { zeroed() };
		panic_on_error!(rust_ibv_query_device_ex, self, &input, &mut attributes);
		attributes
	}
	
	#[inline(always)]
	fn queryRealTimeRawClock(self) -> timespec
	{
		debug_assert!(!self.is_null(), "self is null");
		
		const IBV_VALUES_MASK_RAW_CLOCK: u32 = 1;
		
		let mut realTimeValues = ibv_values_ex
		{
			comp_mask: IBV_VALUES_MASK_RAW_CLOCK,
			raw_clock: unsafe { zeroed() },
		};
		
		panic_on_errno!(rust_ibv_query_rt_values_ex, self, &mut realTimeValues);
		realTimeValues.raw_clock
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn data(self) -> ibv_context
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { *self }
	}
	
	#[inline(always)]
	fn numberOfCompletionVectors(self) -> u32
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let num_comp_vectors = self.data().num_comp_vectors;
		if unlikely(num_comp_vectors < 0)
		{
			0
		}
		else
		{
			num_comp_vectors as u32
		}
	}
	
	#[inline(always)]
	fn commandRawFd(self) -> RawFd
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self.data().cmd_fd
	}
	
	#[inline(always)]
	fn asyncRawFd(self) -> RawFd
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self.data().async_fd
	}
	
	#[inline(always)]
	fn blockOnAsynchronousEvent(self) -> ibv_async_event
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let mut asynchronousEvent = unsafe { uninitialized() };
		panic_on_error!(ibv_get_async_event, self, &mut asynchronousEvent);
		asynchronousEvent
	}
	
	#[inline(always)]
	fn allocateProtectionDomain(self) -> *mut ibv_pd
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_null!(ibv_alloc_pd, self)
	}
	
	#[inline(always)]
	fn createCompletionChannel(self) -> *mut ibv_comp_channel
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_null!(ibv_create_comp_channel, self)
	}
	
	#[inline(always)]
	fn createUnextendedCompletionQueueWithoutCompletionChannel(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self.createUnextendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, null_mut())
	}
	
	#[inline(always)]
	fn createUnextendedCompletionQueue(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, potentiallyNullCompletionChannel: *mut ibv_comp_channel) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		assert!(completionVector < self.numberOfCompletionVectors(), "completionVector '{}' is not less than context numberOfCompletionVectors '{}'", completionVector, self.numberOfCompletionVectors());
		
		panic_on_null!(ibv_create_cq, self, atLeastThisNumberOfCompletionQueueEvents as i32, completionQueueContext, potentiallyNullCompletionChannel, completionVector as i32)
	}
	
	#[inline(always)]
	fn createExtendedCompletionQueueWithoutCompletionChannel(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool) -> *mut ibv_cq_ex
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self.createExtendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, workCompletionFlags, lockLessButNotThreadSafe, null_mut())
	}
	
	#[inline(always)]
	fn createExtendedCompletionQueue(self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool, potentiallyNullCompletionChannel: *mut ibv_comp_channel) -> *mut ibv_cq_ex
	{
		debug_assert!(!self.is_null(), "self is null");
		
		assert!(completionVector < self.numberOfCompletionVectors(), "completionVector '{}' is not less than context numberOfCompletionVectors '{}'", completionVector, self.numberOfCompletionVectors());
		
		let mut attributes = ibv_cq_init_attr_ex
		{
			cqe: atLeastThisNumberOfCompletionQueueEvents,
			cq_context: completionQueueContext,
			channel: potentiallyNullCompletionChannel,
			comp_vector: completionVector,
			wc_flags: workCompletionFlags.0 as u64,
			comp_mask: ibv_cq_init_attr_mask_IBV_CQ_INIT_ATTR_MASK_FLAGS.0,
			flags: if likely(lockLessButNotThreadSafe)
			{
				ibv_create_cq_attr_flags_IBV_CREATE_CQ_ATTR_SINGLE_THREADED.0
			}
			else
			{
				0
			},
		};
		
		panic_on_null!(rust_ibv_create_cq_ex, self, &mut attributes)
	}
}
