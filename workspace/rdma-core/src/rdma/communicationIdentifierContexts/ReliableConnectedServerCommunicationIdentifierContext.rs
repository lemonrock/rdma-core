// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct ReliableConnectedServerCommunicationIdentifierContext;

impl Drop for ReliableConnectedServerCommunicationIdentifierContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
	}
}

macro_rules! ShouldNeverOccur
{
	() =>
	{
		panic!("Should never occur");
	}
}

impl CommunicationIdentifierContext for ReliableConnectedServerCommunicationIdentifierContext
{
}

pub trait CommunicationIdentifierContext: Drop
{
	#[inline(always)]
	fn typeOfService(&self) -> Option<u8>
	{
		unimplemented!();
	}
	
	#[inline(always)]
	fn addressResolutionSucceeded(&self, communicationIdentifier: *mut rdma_cm_id)
	{
		ShouldNeverOccur!();
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn addressResolutionFailed(&self, statusCode: i32) -> bool
	{
		ShouldNeverOccur!();
	}
	
	#[inline(always)]
	fn routeResolutionSucceeded(&self)
	{
		ShouldNeverOccur!();
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn routeResolutionFailed(&self, statusCode: i32)
	{
		ShouldNeverOccur!();
	}
	
	/// u8 => Length of private data written
	/// Return Ok if the request should be accepted
	/// Return Err if the connection should be rejected
	/// Note that private data of length zero will not be transmitted
	/// Note that the supplied buffer is NOT zero'd and information CAN LEAK in. Never do anything other than write to it.
	#[allow(unused_variables)]
	#[inline(always)]
	fn reliableConnectionRequest(&mut self, newCommunicationIdentifierWithNoContextYet: *mut rdma_cm_id, eventData: RequestedConnectionEventData, privateDataBuffer: &mut [u8; 256]) -> (u8, Result<ConnectionAcceptance, ()>)
	{
		// actually rdma_create_qp_ex   https://www.mankier.com/3/ibv_create_qp_ex
		// rdma_create_qp before rdma_accept!
		/*
		void register_memory(struct connection *conn)
{
// use dpdk numa-aware malloc...

  conn->send_region = malloc(BUFFER_SIZE);
  conn->recv_region = malloc(BUFFER_SIZE);

  TEST_Z(conn->send_mr = ibv_reg_mr(
    s_ctx->pd,
    conn->send_region,
    BUFFER_SIZE,
    IBV_ACCESS_LOCAL_WRITE | IBV_ACCESS_REMOTE_WRITE));

  TEST_Z(conn->recv_mr = ibv_reg_mr(
    s_ctx->pd,
    conn->recv_region,
    BUFFER_SIZE,
    IBV_ACCESS_LOCAL_WRITE | IBV_ACCESS_REMOTE_WRITE));
}
		*/
		
		(0, Err(()))
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn unreliableDatagramConnectionRequest(&mut self, newCommunicationIdentifierWithNoContextYet: *mut rdma_cm_id, privateDataBuffer: &mut [u8; 256]) -> (u8, Result<ConnectionAcceptance, ()>)
	{
		(0, Err(()))
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
		// eg post some data
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn multicastEstablished(&self, eventData: UnreliableDatagramEventData)
	{
		ShouldNeverOccur!();
	}
	
	#[inline(always)]
	fn disconnected(&self)
	{
		// destroy queue pair: rdma_destroy_qp
//		ibv_dereg_mr(conn->send_mr);
//		ibv_dereg_mr(conn->recv_mr);
//		free(conn->send_region);
//		free(conn->recv_region);
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn deviceRemoval(&self, transportSpecificCode: i32)
	{
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn multicastJoin(&self, eventData: UnreliableDatagramEventData)
	{
		ShouldNeverOccur!();
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn multicastError(&self, statusCode: i32)
	{
		ShouldNeverOccur!();
	}
	
	#[allow(unused_variables)]
	#[inline(always)]
	fn addressChange(&self, transportSpecificCode: i32)
	{
	}
	
	#[inline(always)]
	fn timeWaitExit(&self)
	{
	}
}
