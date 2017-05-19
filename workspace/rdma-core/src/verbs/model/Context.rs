// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct Context(pub(crate) *mut ibv_context, ibv_device_attr_ex);

impl Drop for Context
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.0.destroy();
	}
}

impl Context
{
	#[inline(always)]
	fn new(pointer: *mut ibv_context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Context(pointer, pointer.extendedAttributes())
	}
	
	#[inline(always)]
	pub fn port<'a>(&'a self, portNumber: u8) -> Port<'a>
	{
		debug_assert!(portNumber < self.numberOfPhysicalPorts(), "portNumber '{}' exceeds maximum number of ports '{}'", portNumber, self.numberOfPhysicalPorts());
		
		Port::new(self, portNumber)
	}
	
	#[inline(always)]
	pub fn data(&self) -> ibv_context
	{
		self.0.data()
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
	pub fn queryRealTimeRawClock(&self) -> timespec
	{
		self.0.queryRealTimeRawClock()
	}
	
	#[inline(always)]
	pub fn deviceHasCapability(&self, capability: ibv_device_cap_flags) -> bool
	{
		self.attributes().deviceHasCapability(capability)
	}
	
	#[inline(always)]
	pub fn numberOfCompletionVectors(&self) -> u32
	{
		self.0.numberOfCompletionVectors()
	}
	
	#[inline(always)]
	pub fn commandFileDescriptor(&self) -> FileDescriptor
	{
		self.0.commandFileDescriptor()
	}
	
	#[inline(always)]
	pub fn asyncFileDescriptor(&self) -> FileDescriptor
	{
		self.0.asyncFileDescriptor()
	}
	
	#[inline(always)]
	pub fn numberOfPhysicalPorts(&self) -> u8
	{
		self.attributes().numberOfPhysicalPorts()
	}
	
	/// See <https://linux.die.net/man/3/ibv_get_async_event>
	#[inline(always)]
	pub fn blockOnAsynchronousEvent(&self) -> AsynchronousEvent
	{
		AsynchronousEvent(self.0.blockOnAsynchronousEvent())
	}
	
	#[inline(always)]
	pub fn allocateProtectionDomain<'a>(&'a self) -> ProtectionDomain<'a>
	{
		ProtectionDomain::new(self.0.allocateProtectionDomain(), self)
	}
	
	#[inline(always)]
	pub fn createCompletionChannel<'a>(&'a self) -> CompletionChannel<'a>
	{
		CompletionChannel::new(self.0.createCompletionChannel(), self)
	}
	
	#[inline(always)]
	pub fn createUnextendedCompletionQueueWithoutCompletionChannel<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> WithoutCompletionChannelUnextendedCompletionQueue<'a>
	{
		let pointer = self.0.createUnextendedCompletionQueueWithoutCompletionChannel(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector);
		WithoutCompletionChannelUnextendedCompletionQueue::new(pointer, self)
	}
	
	#[inline(always)]
	pub fn createExtendedCompletionQueueWithoutCompletionChannel<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool) -> WithoutCompletionChannelExtendedCompletionQueue<'a>
	{
		let pointer = self.0.createExtendedCompletionQueueWithoutCompletionChannel(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, workCompletionFlags, lockLessButNotThreadSafe);
		WithoutCompletionChannelExtendedCompletionQueue::new(pointer, self)
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
	
	/// NOTE: This implementation is almost certainly currently broken
	/// The created object should be used as part of ibv_create_qp_ex() to enable dispatching of incoming packets based on some RX hash configuration.
	#[inline(always)]
	pub fn createReceiveWorkQueueIndirectionTable<'a>(&'a self, size: PowerOfTwoThirtyTwoBit) -> ReceiveWorkQueueIndirectionTable<'a>
	{
		let sizeU32 = size.as_u32();
		
		let mut indirectionTable = Vec::with_capacity(sizeU32 as usize);
		
		let mut attributes = ibv_rwq_ind_table_init_attr
		{
			log_ind_tbl_size: sizeU32,
			ind_tbl: indirectionTable.as_mut_ptr(),
			comp_mask: 0,
		};
		
		let pointer = panic_on_null!(rust_ibv_create_rwq_ind_table, self.0, &mut attributes);
		ReceiveWorkQueueIndirectionTable
		{
			pointer: pointer,
			context: self,
			indirectionTable: indirectionTable,
		}
	}
}
