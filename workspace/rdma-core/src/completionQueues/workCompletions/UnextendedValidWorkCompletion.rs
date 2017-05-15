// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


// This means that all relevant memory buffers are free to use again
pub struct UnextendedValidWorkCompletion<'a>
{
	unextendedWorkCompletion: &'a UnextendedWorkCompletion
}

impl<'a> UnextendedValidWorkCompletion<'a>
{
	#[inline(always)]
	pub fn workRequestOperationWas(&self) -> ibv_wc_opcode
	{
		self.unextendedWorkCompletion.0.opcode
	}
	
	// Only relevant for UD => Unreliable datagram?
	// AKA source queue pair number
	#[inline(always)]
	pub fn receiveWorkRequestRemoteQueuePairNumber(&self) -> QueuePairNumber
	{
		self.unextendedWorkCompletion.0.src_qp
	}
	
	#[inline(always)]
	pub fn immediateDataInNetworkByteOrder(&self) -> Option<u32>
	{
		const IBV_WC_WITH_IMM: c_int = 2;
		if unlikely(self.unextendedWorkCompletion.0.wc_flags & IBV_WC_WITH_IMM == IBV_WC_WITH_IMM)
		{
			let workRequestOperation = self.workRequestOperationWas();
			if likely(workRequestOperation == ibv_wc_opcode::IBV_WC_SEND || workRequestOperation == ibv_wc_opcode::IBV_WC_RDMA_WRITE)
			{
				let union = (self.unextendedWorkCompletion.0).__bindgen_anon_1;
				return Some(unsafe { union.imm_data });
			}
		}
		None
	}
	
	// Only relevant for UD; all UD have a 40 byte reserved space at the beginning
	#[inline(always)]
	pub fn validGlobalRoutingHeaderPresentInFirst40Bytes(&self) -> bool
	{
		const IBV_WC_GRH: c_int = 1;
		self.unextendedWorkCompletion.0.wc_flags & IBV_WC_GRH == IBV_WC_GRH
	}
	
	// Only relevant for UD
	#[inline(always)]
	pub fn receiveWorkRequestSourceLocalIdentifier(&self) -> LocalIdentifier
	{
		self.unextendedWorkCompletion.0.slid
	}
	
	// Only relevant for UD
	#[inline(always)]
	pub fn receiveWorkRequestServiceLevel(&self) -> ServiceLevel
	{
		unsafe { transmute(self.unextendedWorkCompletion.0.sl) }
	}
	
	// Only relevant for UD and only then for unicast messages
	#[inline(always)]
	pub fn receiveWorkRequestDestinationLocalIdentifierPath(&self) -> LocalIdentifierPath
	{
		self.unextendedWorkCompletion.0.dlid_path_bits
	}
	
	/// The number of bytes transferred. Relevant if the Receive Queue for incoming Send or RDMA Write with immediate operations. This value doesn't include the length of the immediate data, if such exists. Relevant in the Send Queue for RDMA Read and Atomic operations.
	/// For the Receive Queue of a UD QP that is not associated with an SRQ or for an SRQ that is associated with a UD QP this value equals to the payload of the message plus the 40 bytes reserved for the GRH.
	/// The number of bytes transferred is the payload of the message plus the 40 bytes reserved for the GRH, whether or not the GRH is present
	#[inline(always)]
	pub fn numberOfBytesTransferred(&self) -> u32
	{
		self.unextendedWorkCompletion.0.byte_len as u32
	}
}
