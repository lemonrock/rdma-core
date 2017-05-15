// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct UnextendedValidWorkCompletion(ibv_wc);

impl<'a> ValidWorkCompletion<'a> for UnextendedValidWorkCompletion
{
	#[inline(always)]
	fn flags(&self) -> c_int
	{
		self.0.wc_flags
	}
	
	#[inline(always)]
	fn workRequestOperationWas(&self) -> ibv_wc_opcode
	{
		self.0.opcode
	}
	
	// Only relevant for UD => Unreliable datagram?
	// AKA source queue pair number
	#[inline(always)]
	fn receiveWorkRequestRemoteQueuePairNumber(&self) -> QueuePairNumber
	{
		self.0.src_qp
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn rawImmediateDataInNetworkByteOrder(&self) -> u32
	{
		let union = (self.0).__bindgen_anon_1;
		unsafe { union.imm_data }
	}
	
	// Only relevant for UD
	#[inline(always)]
	fn receiveWorkRequestSourceLocalIdentifier(&self) -> LocalIdentifier
	{
		self.0.slid
	}
	
	// Only relevant for UD
	#[inline(always)]
	fn receiveWorkRequestServiceLevel(&self) -> ServiceLevel
	{
		unsafe { transmute(self.0.sl) }
	}
	
	// Only relevant for UD and only then for unicast messages
	#[inline(always)]
	fn receiveWorkRequestDestinationLocalIdentifierPath(&self) -> LocalIdentifierPath
	{
		self.0.dlid_path_bits
	}
	
	/// The number of bytes transferred. Relevant if the Receive Queue for incoming Send or RDMA Write with immediate operations. This value doesn't include the length of the immediate data, if such exists. Relevant in the Send Queue for RDMA Read and Atomic operations.
	/// For the Receive Queue of a UD QP that is not associated with an SRQ or for an SRQ that is associated with a UD QP this value equals to the payload of the message plus the 40 bytes reserved for the GRH.
	/// The number of bytes transferred is the payload of the message plus the 40 bytes reserved for the GRH, whether or not the GRH is present
	#[inline(always)]
	fn numberOfBytesTransferred(&self) -> u32
	{
		self.0.byte_len as u32
	}
	
	#[inline(always)]
	fn partitionKeyIndex(&self) -> PartitionKeyIndex
	{
		self.0.pkey_index
	}
}
