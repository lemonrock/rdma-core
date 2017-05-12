// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct Context(*mut ibv_context, ibv_device_attr);

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
		
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ibv_query_device, pointer, &mut attributes);
		
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
		&self.1
	}
	
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
	pub fn createCompletionQueueWithoutCompletionChannel<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32) -> CompletionQueue<'a>
	{
		self.createCompletionQueueInternal(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, completionVector, None)
	}
	
	#[inline(always)]
	fn createCompletionQueueInternal<'a>(&'a self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, completionVector: u32, completionChannel: Option<&'a CompletionChannel<'a>>) -> CompletionQueue<'a>
	{
		debug_assert!(completionVector < self.numberOfCompletionVectors(), "completionVector '{}' is not less than context numberOfCompletionVectors '{}'", completionVector, self.numberOfCompletionVectors());
		
		let potentiallyNullCompletionChannel = match completionChannel
		{
			None => null_mut(),
			Some(value) => value.pointer,
		};
		let completionQueuePointer = panic_on_null!(ibv_create_cq, self.0, atLeastThisNumberOfCompletionQueueEvents as i32, completionQueueContext, potentiallyNullCompletionChannel, completionVector as i32);
		
		CompletionQueue::new(completionQueuePointer, completionChannel)
	}
	
	/*
		pub fn ibv_resolve_eth_l2_from_gid(context: *mut ibv_context, attr: *mut ibv_ah_attr, eth_mac: *mut u8, vid: *mut u16) -> c_int;
	*/
}