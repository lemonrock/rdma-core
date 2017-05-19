// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ValidWorkCompletion
{
	#[inline(always)]
	fn flags(&self) -> c_int;
	
	#[inline(always)]
	fn hasFlag(&self, flag: c_int) -> bool
	{
		self.flags() & flag == flag
	}
	
	#[inline(always)]
	fn workRequestOperationWas(&self) -> ibv_wc_opcode;
	
	// Only relevant for UD => Unreliable datagram?
	// AKA source queue pair number
	#[inline(always)]
	fn receiveWorkRequestRemoteQueuePairNumber(&self) -> QueuePairNumber;
	
	#[inline(always)]
	fn immediateDataInNetworkByteOrder(&self) -> Option<u32>
	{
		const IBV_WC_WITH_IMM: c_int = 2;
		if unlikely(self.hasFlag(IBV_WC_WITH_IMM))
		{
			let workRequestOperation = self.workRequestOperationWas();
			if likely(workRequestOperation == ibv_wc_opcode::IBV_WC_SEND || workRequestOperation == ibv_wc_opcode::IBV_WC_RDMA_WRITE)
			{
				return Some(self.rawImmediateDataInNetworkByteOrder());
			}
		}
		None
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn rawImmediateDataInNetworkByteOrder(&self) -> u32;
	
	// Only relevant for UD; all UD have a 40 byte reserved space at the beginning
	#[inline(always)]
	fn validGlobalRoutingHeaderPresentInFirst40Bytes(&self) -> bool
	{
		const IBV_WC_GRH: c_int = 1;
		self.hasFlag(IBV_WC_GRH)
	}
	
	// Only relevant for UD
	#[inline(always)]
	fn receiveWorkRequestSourceLocalIdentifier(&self) -> LocalIdentifier;
	
	// Only relevant for UD
	#[inline(always)]
	fn receiveWorkRequestServiceLevel(&self) -> ServiceLevel;
	
	// Only relevant for UD and only then for unicast messages
	#[inline(always)]
	fn receiveWorkRequestDestinationLocalIdentifierPath(&self) -> LocalIdentifierPath;
	
	/// The number of bytes transferred. Relevant if the Receive Queue for incoming Send or RDMA Write with immediate operations. This value doesn't include the length of the immediate data, if such exists. Relevant in the Send Queue for RDMA Read and Atomic operations.
	/// For the Receive Queue of a UD QP that is not associated with an SRQ or for an SRQ that is associated with a UD QP this value equals to the payload of the message plus the 40 bytes reserved for the GRH.
	/// The number of bytes transferred is the payload of the message plus the 40 bytes reserved for the GRH, whether or not the GRH is present
	#[inline(always)]
	fn numberOfBytesTransferred(&self) -> u32;
	
	#[inline(always)]
	fn partitionKeyIndex(&self) -> PartitionKeyIndex;
}
