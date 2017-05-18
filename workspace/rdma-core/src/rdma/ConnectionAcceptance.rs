// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConnectionAcceptance
{
	/// This value must be less than or equal to the local RDMA device attribute max_qp_rd_atom, but preferably greater than or equal to the responder_resources value reported in the connect request event.
	responderResourcesAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsRemotely: u8,
	
	/// This value must be less than or equal to the local RDMA device attribute max_qp_init_rd_atom and the initiator_depth value reported in the connect request event.
	initiatorDepthAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsLocally: u8,
	
	hardwareFlowControlAvailable: bool,
	
	maximumNumberOfRetriesForARemoteSendToUs: u8,
	
	sharedReceiveQueuePresent: bool,
	
	queuePairNumber: QueuePairNumber,
}

impl ConnectionAcceptance
{
	#[inline(always)]
	pub fn reliableConnectionAcceptance
	(
		responderResourcesAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsRemotely: u8,
		initiatorDepthAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsLocally: u8,
		hardwareFlowControlAvailable: bool,
		maximumNumberOfRetriesForARemoteSendToUs: u8,
		sharedReceiveQueuePresent: bool,
		queuePairNumber: QueuePairNumber,
	) -> Self
	{
		Self
		{
			responderResourcesAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsRemotely,
			initiatorDepthAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsLocally,
			hardwareFlowControlAvailable,
			maximumNumberOfRetriesForARemoteSendToUs,
			sharedReceiveQueuePresent,
			queuePairNumber,
		}
	}
	
	#[inline(always)]
	pub fn unreliableDatagramConnectionAcceptance(queuePairNumber: QueuePairNumber) -> Self
	{
		Self
		{
			responderResourcesAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsRemotely: 0,
			initiatorDepthAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsLocally: 0,
			hardwareFlowControlAvailable: false,
			maximumNumberOfRetriesForARemoteSendToUs: 0,
			sharedReceiveQueuePresent: false,
			queuePairNumber: queuePairNumber,
		}
	}
	
	#[inline(always)]
	fn populate(&self, privateDataBuffer: &[u8; 256], privateDataLength: u8, parameter: &mut rdma_conn_param)
	{
		parameter.private_data = if privateDataLength == 0
		{
			null()
		}
		else
		{
			privateDataBuffer as *const u8 as *const _
		};
		parameter.private_data_len = privateDataLength;
		parameter.responder_resources = self.responderResourcesAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsRemotely;
		parameter.initiator_depth = self.initiatorDepthAlsoKnownAsMaximumOutstandingRdmaReadAndAtomicOperationsLocally;
		parameter.flow_control = if likely(self.hardwareFlowControlAvailable)
		{
			1
		}
		else
		{
			0
		};
		parameter.retry_count = 0; // Apparently ignored
		parameter.rnr_retry_count = self.maximumNumberOfRetriesForARemoteSendToUs;
		parameter.srq = if likely(self.sharedReceiveQueuePresent)
		{
			1
		}
		else
		{
			0
		};
		parameter.qp_num = self.queuePairNumber;
	}
}
