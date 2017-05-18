// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ConnectionEventData : EventData
{
	#[doc(hidden)]
	#[inline(always)]
	fn data(&self) -> rdma_conn_param
	{
		unsafe { (*self.pointer()).param.conn }
	}
	
	#[inline(always)]
	fn responderResourcesRequestedAlsoKnownAsRemoteNodeInitiatorDepth(&self) -> u8
	{
		self.data().responder_resources
	}
	
	/// The maximum number of outstanding RDMA read / atomic operations that the recipient (us) may have outstanding
	#[inline(always)]
	fn initiatorDepthAlsoKnownAsRemoteNodeResponderResourcesRequested(&self) -> u8
	{
		self.data().initiator_depth
	}
	
	#[inline(always)]
	fn hardwareLevelFlowControlIsProvidedByTheSender(&self) -> bool
	{
		self.data().flow_control != 0
	}
	
	/// Abbreviations:-
	/// - Receiver Not Ready, RNR
	/// - Negative Acknowledgments: NACK
	#[inline(always)]
	fn numberOfTimesWeShouldRetryReceiverNotReadyNegativeAcknowledgments(&self) -> u8
	{
		self.data().rnr_retry_count
	}
	
	#[inline(always)]
	fn senderIsUsingASharedReceiveQueue(&self) -> bool
	{
		self.data().srq != 0
	}
}
