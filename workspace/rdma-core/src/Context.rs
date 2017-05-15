// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct Context(*mut ibv_context, ibv_device_attr_ex);

impl Drop for Context
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let result = unsafe { ibv_close_device(self.0) };
		debug_assert!(result == 0 || result == 1, "ibv_close_device returned a result '{}' which was not 0 or 1", result);
		if unlikely(result == -1)
		{
			// Examination of C source code suggests it can't actually fail
			panic!("ibv_close_device failed (no further details)");
		}
	}
}

impl Context
{
	#[inline(always)]
	fn new(pointer: *mut ibv_context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		//let mut attributes = unsafe { uninitialized() };
		let mut attributes = unsafe { zeroed() };
		panic_on_error!(rust_ibv_query_device_ex, pointer, null_mut(), &mut attributes);
		Context(pointer, attributes)
	}
	
	#[inline(always)]
	fn data(&self) -> ibv_context
	{
		unsafe { *self.0 }
	}
	
	/// See <https://linux.die.net/man/3/ibv_query_device> for explanations of fields of ibv_device_attr
	#[inline(always)]
	pub fn attributes(&self) -> &ibv_device_attr
	{
		&self.1.orig_attr
	}
	
	#[inline(always)]
	pub fn extendedAttributes(&self) -> &ibv_device_attr_ex
	{
		&self.1
	}
	
	#[inline(always)]
	pub fn queryRealTimeValues(&self) -> ibv_values_ex
	{
		const IBV_VALUES_MASK_RAW_CLOCK: u32 = 1;
		
		let mut realTimeValues = ibv_values_ex
		{
			comp_mask: IBV_VALUES_MASK_RAW_CLOCK,
			raw_clock: unsafe { zeroed() },
		};
		
		panic_on_errno!(rust_ibv_query_rt_values_ex, self.0, &mut realTimeValues);
		realTimeValues
	}
	
	/*
	
pub fn rust_ibv_create_rwq_ind_table(context: *mut ibv_context, init_attr: *mut ibv_rwq_ind_table_init_attr) -> *mut ibv_rwq_ind_table;
pub fn rust_ibv_create_wq(context: *mut ibv_context, wq_init_attr: *mut ibv_wq_init_attr) -> *mut ibv_wq;

*/
	
	#[inline(always)]
	pub fn deviceHasCapability(&self, capability: ibv_device_cap_flags) -> bool
	{
		let bit = capability as i32;
		self.attributes().device_cap_flags & bit == bit
	}
	
	#[inline(always)]
	pub fn numberOfCompletionVectors(&self) -> u32
	{
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
	pub fn commandFileDescriptor(&self) -> FileDescriptor
	{
		self.data().cmd_fd
	}
	
	#[inline(always)]
	pub fn asyncFileDescriptor(&self) -> FileDescriptor
	{
		self.data().async_fd
	}
	
	#[inline(always)]
	pub fn numberOfPhysicalPorts(&self) -> u8
	{
		self.attributes().phys_port_cnt
	}
	
	/// See <https://linux.die.net/man/3/ibv_get_async_event>
	#[inline(always)]
	pub fn blockOnAsynchronousEvent(&self) -> AsynchronousEvent
	{
		let mut asynchronousEvent = unsafe { uninitialized() };
		panic_on_error!(ibv_get_async_event, self.0, &mut asynchronousEvent);
		AsynchronousEvent(asynchronousEvent)
	}
	
	#[inline(always)]
	pub fn port<'a>(&'a self, portNumber: u8) -> Port<'a>
	{
		debug_assert!(portNumber < self.numberOfPhysicalPorts(), "portNumber '{}' exceeds maximum number of ports '{}'", portNumber, self.numberOfPhysicalPorts());
		
		Port::new(self, portNumber)
	}
	
	#[inline(always)]
	pub fn allocateProtectionDomain<'a>(&'a self) -> ProtectionDomain<'a>
	{
		ProtectionDomain::new(panic_on_null!(ibv_alloc_pd, self.0), self)
	}
	
	#[inline(always)]
	pub fn createCompletionChannel<'a>(&'a self) -> CompletionChannel<'a>
	{
		CompletionChannel::new(panic_on_null!(ibv_create_comp_channel, self.0), self)
	}
	
	#[inline(always)]
	pub fn createUnextendedCompletionQueueWithoutCompletionChannel<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> WithoutCompletionChannelUnextendedCompletionQueue<'a>
	{
		let pointer = self.createUnextendedCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, null_mut());
		WithoutCompletionChannelUnextendedCompletionQueue::new(pointer, self)
	}
	
	#[inline(always)]
	fn createUnextendedCompletionQueueInternal<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, potentiallyNullCompletionChannel: *mut ibv_comp_channel) -> *mut ibv_cq
	{
		debug_assert!(completionVector < self.numberOfCompletionVectors(), "completionVector '{}' is not less than context numberOfCompletionVectors '{}'", completionVector, self.numberOfCompletionVectors());
		
		panic_on_null!(ibv_create_cq, self.0, atLeastThisNumberOfCompletionQueueEvents as i32, completionQueueContext, potentiallyNullCompletionChannel, completionVector as i32)
	}
	
	#[inline(always)]
	pub fn createExtendedCompletionQueueWithoutCompletionChannel<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool) -> WithoutCompletionChannelExtendedCompletionQueue<'a>
	{
		let pointer = self.createExtendedCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, workCompletionFlags, lockLessButNotThreadSafe, null_mut());
		WithoutCompletionChannelExtendedCompletionQueue::new(pointer, self)
	}
	
	#[inline(always)]
	fn createExtendedCompletionQueueInternal<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool, potentiallyNullCompletionChannel: *mut ibv_comp_channel) -> *mut ibv_cq_ex
	{
		debug_assert!(completionVector < self.numberOfCompletionVectors(), "completionVector '{}' is not less than context numberOfCompletionVectors '{}'", completionVector, self.numberOfCompletionVectors());
		
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
		
		panic_on_null!(rust_ibv_create_cq_ex, self.0, &mut attributes)
	}
	
	#[inline(always)]
	pub fn createExtendedReliableConnectionDomainWithoutInode<'a>(&'a self) -> ExtendedReliableConnectionDomain<'a>
	{
		let pointer = self.openExtendedReliableConnectionDomainInternal(-1, true, false);
		if unlikely(pointer.is_null())
		{
			let errno = errno();
			panic!("{} failed with error number '{}' ('{}')", stringify!($function), errno.0, errno);
		}
		ExtendedReliableConnectionDomain
		{
			pointer: pointer,
			context: self,
		}
	}
	
	#[inline(always)]
	fn openExtendedReliableConnectionDomainInternal<'a>(&'a self, fileDescriptor: FileDescriptor, create: bool, exclusive: bool) -> *mut ibv_xrcd
	{
		const IBV_XRCD_INIT_ATTR_FD: u32 = 1;
		const IBV_XRCD_INIT_ATTR_OFLAGS: u32 = 2;
		
		const AllCurrentFields: u32 = IBV_XRCD_INIT_ATTR_FD | IBV_XRCD_INIT_ATTR_OFLAGS;
		
		let mut openFlags = 0;
		if create
		{
			openFlags = openFlags | O_CREAT;
		}
		if exclusive
		{
			openFlags = openFlags | O_EXCL;
		}
		
		if unlikely(fileDescriptor == -1 && !create)
		{
			panic!("create must be true if fileDescriptor is -1");
		}
		if unlikely(fileDescriptor == -1 && exclusive)
		{
			panic!("exclusive must be false if fileDescriptor is -1");
		}
		
		let mut attributes = ibv_xrcd_init_attr
		{
			comp_mask: AllCurrentFields,
			fd: fileDescriptor,
			oflags: openFlags,
		};
		
		unsafe { rust_ibv_open_xrcd(self.0, &mut attributes) }
	}
}
