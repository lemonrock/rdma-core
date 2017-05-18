// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait rdma_event_channelEx
{
	#[inline(always)]
	fn fileDescriptorForEPoll(self) -> FileDescriptor;
	
	#[inline(always)]
	fn destroy(self);
	
	/// self can be null
	#[inline(always)]
	fn newListeningCommunicationIdentifier(self, context: Box<CommunicationIdentifierContext>, addressing: Addressing, backLog: u32) -> *mut rdma_cm_id;
	
	#[inline(always)]
	fn getEvent(self) -> *mut rdma_cm_event;
}

impl rdma_event_channelEx for *mut rdma_event_channel
{
	#[inline(always)]
	fn fileDescriptorForEPoll(self) -> FileDescriptor
	{
		unsafe { (*self).fd }
	}
	
	#[inline(always)]
	fn destroy(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { rdma_destroy_event_channel(self) }
	}
	
	#[inline(always)]
	fn newListeningCommunicationIdentifier(self, context: Box<CommunicationIdentifierContext>, addressing: Addressing, backLog: u32) -> *mut rdma_cm_id
	{
		let contextThinPointer = Box::new(context);
		
		let (mut rdmaSocketAddress, portSpace) = addressing.createForLocal();
		
		let mut this = unsafe { uninitialized() };
		#[allow(trivial_casts)]
		{
			panic_on_error!(rdma_create_id, self, &mut this, Box::into_raw(contextThinPointer) as *mut _ as *mut c_void, portSpace);
		}
		
		panic_on_error!(rdma_bind_addr, this, rdmaSocketAddress.as_sockaddr_mut_ptr());
		
		panic_on_error!(rdma_listen, this, backLog as c_int);
		
		this
	}
	
	#[inline(always)]
	fn getEvent(self) -> *mut rdma_cm_event
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let mut event = unsafe { uninitialized() };
		panic_on_error!(rdma_get_cm_event, self, &mut event);
		debug_assert!(!event.is_null(), "event was null - this should not occur");
		event
	}
}
