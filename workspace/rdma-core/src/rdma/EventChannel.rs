// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct EventChannel<C: CommunicationEventHandler>
{
	pointer: *mut rdma_event_channel,
	communicationEventHandler: C,
}

impl<C: CommunicationEventHandler> Drop for EventChannel<C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { rdma_destroy_event_channel(self.pointer) }
	}
}

impl<C: CommunicationEventHandler> EventChannel<C>
{
	#[inline(always)]
	pub fn new(communicationEventHandler: C) -> Self
	{
		EventChannel
		{
			pointer: panic_on_null!(rdma_create_event_channel),
			communicationEventHandler: communicationEventHandler,
		}
	}
	
	#[inline(always)]
	pub fn createAsynchronousCommunicationIdentifier<Context>(&self, portSpace: rdma_port_space, context: Rc<RefCell<Context>>) -> AsynchronousCommunicationIdentifier<Context>
	{
		let mut communicationIdentifierPointer = unsafe { uninitialized() };
		panic_on_error!(rdma_create_id, self.pointer, &mut communicationIdentifierPointer, Rc::into_raw(context) as *mut RefCell<Context> as *mut c_void, portSpace);
		AsynchronousCommunicationIdentifier::new(communicationIdentifierPointer)
	}
	
	#[inline(always)]
	pub fn blockWaitingForCommunicationEvent(&self)
	{
		let mut eventPointer = unsafe { uninitialized() };
		panic_on_error!(rdma_get_cm_event, self.pointer, &mut eventPointer);
		
		match unsafe { *eventPointer }.event
		{
			// rdma_resolve_addr()
			rdma_cm_event_type::RDMA_CM_EVENT_ADDR_RESOLVED => self.communicationEventHandler.addressResolutionSucceeded(),
			rdma_cm_event_type::RDMA_CM_EVENT_ADDR_ERROR => self.communicationEventHandler.addressResolutionFailed(),
			
			// rdma_resolve_route()
			rdma_cm_event_type::RDMA_CM_EVENT_ROUTE_RESOLVED => self.communicationEventHandler.routeResolutionSucceeded(),
			rdma_cm_event_type::RDMA_CM_EVENT_ROUTE_ERROR => self.communicationEventHandler.routeResolutionFailed(),
			
			// Passive (server) side
			rdma_cm_event_type::RDMA_CM_EVENT_CONNECT_REQUEST => self.communicationEventHandler.connectionRequest(),
			
			// Active (client) side (Documentation: "It is only generated on rdma_cm_id's that do not have a QP associated with them")
			rdma_cm_event_type::RDMA_CM_EVENT_CONNECT_RESPONSE => self.communicationEventHandler.connectionResponse(),
			
			// Passive OR Active side
			rdma_cm_event_type::RDMA_CM_EVENT_CONNECT_ERROR => self.communicationEventHandler.connectionError(),
			
			// Active (client) side (Documentation: "If this event is generated in response to a UD QP resolution request over InfiniBand, the event status field will contain an errno, if negative, or the status result carried in the IB CM SIDR REP message.")
			rdma_cm_event_type::RDMA_CM_EVENT_UNREACHABLE => self.communicationEventHandler.unreachable(),
			
			// "Indicates that a connection request or response was rejected by the remote end point.  The event status field will contain the transport specific reject reason if available.  Under InfiniBand, this is the reject reason carried in the IB CM REJ message."
			rdma_cm_event_type::RDMA_CM_EVENT_REJECTED => self.communicationEventHandler.rejected(),
			
			rdma_cm_event_type::RDMA_CM_EVENT_ESTABLISHED => self.communicationEventHandler.established(),
			rdma_cm_event_type::RDMA_CM_EVENT_DISCONNECTED => self.communicationEventHandler.disconnected(),
			
			// "Upon receiving this event, the user must destroy the related rdma_cm_id."
			rdma_cm_event_type::RDMA_CM_EVENT_DEVICE_REMOVAL => self.communicationEventHandler.deviceRemoval(),
			rdma_cm_event_type::RDMA_CM_EVENT_MULTICAST_JOIN => self.communicationEventHandler.multicastJoin(),
			rdma_cm_event_type::RDMA_CM_EVENT_MULTICAST_ERROR => self.communicationEventHandler.multicastError(),
			
			// "The network device associated with this ID through address resolution changed its HW address, eg following of bonding failover. This event can serve as a hint for applications who want the links used for their RDMA sessions to align with the network stack"
			rdma_cm_event_type::RDMA_CM_EVENT_ADDR_CHANGE => self.communicationEventHandler.addressChange(),
			
			// "The QP associated with a connection has exited its timewait state and is now ready to be re-used.  After a QP has been disconnected, it is maintained in a timewait state to allow any in flight packets to exit the network.  After the timewait state has completed, the rdma_cm will report this event."
			rdma_cm_event_type::RDMA_CM_EVENT_TIMEWAIT_EXIT => self.communicationEventHandler.timeWaitExit(),
		}
		
		panic_on_error!(rdma_ack_cm_event, eventPointer)
	}
}
