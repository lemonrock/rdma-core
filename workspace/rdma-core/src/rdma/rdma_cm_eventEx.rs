// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait rdma_cm_eventEx
{
	/// Event MUST have been ack'd before a rdma_cm_id is destroyed (dropped)
	#[inline(always)]
	fn acknowledge(self);
	
	#[inline(always)]
	fn communicationIdentifier(self) -> *mut rdma_cm_id;
	
	#[inline(always)]
	fn listenerCommunicationIdentifier(self) -> *mut rdma_cm_id;
	
	#[inline(always)]
	fn eventType(self) -> rdma_cm_event_type;
	
	#[inline(always)]
	fn statusCode(self) -> i32;
	
	#[inline(always)]
	fn nonZeroStatusCode(self) -> i32;
	
	#[inline(always)]
	fn reconstituteContext(self) -> Box<Box<CommunicationIdentifierContext>>;
	
	#[inline(always)]
	fn reconstituteListenerContext(self) -> Box<Box<CommunicationIdentifierContext>>;
	
	#[inline(always)]
	fn portSpace(self) -> rdma_port_space;
}

impl rdma_cm_eventEx for *mut rdma_cm_event
{
	#[inline(always)]
	fn acknowledge(self)
	{
		panic_on_error!(rdma_ack_cm_event, self);
	}
	
	#[inline(always)]
	fn communicationIdentifier(self) -> *mut rdma_cm_id
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { *self }.id
	}
	
	#[inline(always)]
	fn listenerCommunicationIdentifier(self) -> *mut rdma_cm_id
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { *self }.listen_id
	}
	
	#[inline(always)]
	fn eventType(self) -> rdma_cm_event_type
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { *self }.event
	}
	
	#[inline(always)]
	fn statusCode(self) -> i32
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { *self }.status
	}
	
	#[inline(always)]
	fn nonZeroStatusCode(self) -> i32
	{
		let statusCode = self.statusCode();
		debug_assert!(statusCode != 0);
		statusCode
	}
	
	#[inline(always)]
	fn reconstituteContext(self) -> Box<Box<CommunicationIdentifierContext>>
	{
		let communicationIdentifier = self.communicationIdentifier();
		debug_assert!(!communicationIdentifier.is_null(), "communicationIdentifier is null");
		communicationIdentifier.reconstituteContext()
	}
	
	#[inline(always)]
	fn reconstituteListenerContext(self) -> Box<Box<CommunicationIdentifierContext>>
	{
		let listenerCommunicationIdentifier = self.listenerCommunicationIdentifier();
		debug_assert!(!listenerCommunicationIdentifier.is_null(), "listenerCommunicationIdentifier is null");
		listenerCommunicationIdentifier.reconstituteContext()
	}
	
	#[inline(always)]
	fn portSpace(self) -> rdma_port_space
	{
		let communicationIdentifier = self.communicationIdentifier();
		debug_assert!(!communicationIdentifier.is_null(), "communicationIdentifier is null");
		communicationIdentifier.portSpace()
	}
}
