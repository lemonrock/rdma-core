// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CommunicationIdentifierContext: Drop
{
	#[inline(always)]
	fn addressResolutionSucceeded(&self)
	{
	}
	
	/// It is strongly believed that statusCode is non-zero; this is checked by the caller
	#[allow(unused_variables)]
	#[inline(always)]
	fn addressResolutionFailed(&self, statusCode: i32)
	{
	}
	
	#[inline(always)]
	fn routeResolutionSucceeded(&self)
	{
	}
	
	/// It is strongly believed that statusCode is non-zero; this is checked by the caller
	#[allow(unused_variables)]
	#[inline(always)]
	fn routeResolutionFailed(&self, statusCode: i32)
	{
	}
	
	/// Return true if this connection request should be destroyed
	#[allow(unused_variables)]
	#[inline(always)]
	fn connectionRequest(&self, newCommunicationIdentifierWithNoContextYet: *mut rdma_cm_id, eventData: RequestedConnectionEventData) -> bool
	{
		true
	}
	
	#[inline(always)]
	fn connectionResponse(&self)
	{
	}
	
	/// Active (client) side (Documentation: "If this event is generated in response to a UD QP resolution request over InfiniBand, the event status field will contain an errno, if negative, or the status result carried in the IB CM SIDR REP message.")
	#[allow(unused_variables)]
	#[inline(always)]
	fn connectionError(&self, statusCode: i32)
	{
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn unreachable(&self, statusCode: i32)
	{
	}
	
	/// "Indicates that a connection request or response was rejected by the remote end point. The rejectionReason will contain the transport specific reject reason if available. Under InfiniBand, this is the reject reason carried in the IB CM REJ message."
	#[allow(unused_variables)]
	#[inline(always)]
	fn connectionRequestOrResponseRejected(&self, rejectionReasonCode: i32)
	{
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn connectionEstablished(&self, eventData: EstablishedConnectionEventData)
	{
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn multicastEstablished(&self, eventData: UnreliableDatagramEventData)
	{
	}
	
	#[inline(always)]
	fn disconnected(&self)
	{
	}
	
	/// "Upon receiving this event, the user must destroy the related rdma_cm_id."
	/// The caller of this method will do the necessary destruction. This context will be dropped() after this call
	/// It is not clear whether this event occurs before or after disconnected() / rejected() / timeWait() / connectionError(), or instead of it
	/// It is believed that statusCode is non-zero but this is uncertain
	#[allow(unused_variables)]
	#[inline(always)]
	fn deviceRemoval(&self, transportSpecificCode: i32)
	{
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn multicastJoin(&self, eventData: UnreliableDatagramEventData)
	{
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn multicastError(&self, statusCode: i32)
	{
	}
	
	/// "The network device associated with this ID through address resolution changed its HW address, eg following bonding failover. This event can serve as a hint for applications who want the links used for their RDMA sessions to align with the network stack"
	/// It is believed that statusCode is non-zero but this is uncertain
	#[allow(unused_variables)]
	#[inline(always)]
	fn addressChange(&self, transportSpecificCode: i32)
	{
	}
	
	/// "The QP associated with a connection has exited its timewait state and is now ready to be re-used.  After a QP has been disconnected, it is maintained in a timewait state to allow any in flight packets to exit the network.  After the timewait state has completed, the rdma_cm will report this event."
	#[inline(always)]
	fn timeWaitExit(&self)
	{
	}
}
