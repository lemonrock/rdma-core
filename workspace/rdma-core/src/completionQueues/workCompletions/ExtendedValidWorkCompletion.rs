// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ExtendedValidWorkCompletion<'a>(&'a ExtendedCompletionQueue<'a>);

impl<'a> ValidWorkCompletion<'a> for ExtendedValidWorkCompletion<'a>
{
	#[inline(always)]
	fn flags(&self) -> c_int
	{
		unsafe { rust_ibv_wc_read_wc_flags(self.0.pointer) }
	}
	
	#[inline(always)]
	fn workRequestOperationWas(&self) -> ibv_wc_opcode
	{
		unsafe { rust_ibv_wc_read_opcode(self.0.pointer) }
	}
	
	// Only relevant for UD => Unreliable datagram?
	// AKA source queue pair number
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_SRC_QP
	#[inline(always)]
	fn receiveWorkRequestRemoteQueuePairNumber(&self) -> QueuePairNumber
	{
		unsafe { rust_ibv_wc_read_src_qp(self.0.pointer) }
	}
	
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_IMM
	#[doc(hidden)]
	#[inline(always)]
	fn rawImmediateDataInNetworkByteOrder(&self) -> u32
	{
		unsafe { rust_ibv_wc_read_imm_data(self.0.pointer) }
	}
	
	// Only relevant for UD
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_SLID
	#[inline(always)]
	fn receiveWorkRequestSourceLocalIdentifier(&self) -> LocalIdentifier
	{
		(unsafe { rust_ibv_wc_read_slid(self.0.pointer) }) as u16
	}
	
	// Only relevant for UD
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_SL
	#[inline(always)]
	fn receiveWorkRequestServiceLevel(&self) -> ServiceLevel
	{
		unsafe { transmute(rust_ibv_wc_read_sl(self.0.pointer)) }
	}
	
	// Only relevant for UD and only then for unicast messages
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_DLID_PATH_BITS
	#[inline(always)]
	fn receiveWorkRequestDestinationLocalIdentifierPath(&self) -> LocalIdentifierPath
	{
		unsafe { rust_ibv_wc_read_dlid_path_bits(self.0.pointer) }
	}
	
	/// The number of bytes transferred. Relevant if the Receive Queue for incoming Send or RDMA Write with immediate operations. This value doesn't include the length of the immediate data, if such exists. Relevant in the Send Queue for RDMA Read and Atomic operations.
	/// For the Receive Queue of a UD QP that is not associated with an SRQ or for an SRQ that is associated with a UD QP this value equals to the payload of the message plus the 40 bytes reserved for the GRH.
	/// The number of bytes transferred is the payload of the message plus the 40 bytes reserved for the GRH, whether or not the GRH is present
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_BYTE_LEN
	#[inline(always)]
	fn numberOfBytesTransferred(&self) -> u32
	{
		unsafe { rust_ibv_wc_read_byte_len(self.0.pointer) }
	}
	
	#[inline(always)]
	fn partitionKeyIndex(&self) -> PartitionKeyIndex
	{
		panic!("The static inline function ibv_wc_read_pkey_index is documented but not implemented by libibverbs");
	}
}

impl<'a>  ExtendedValidWorkCompletion<'a>
{
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_COMPLETION_TIMESTAMP
	#[inline(always)]
	pub fn completionTimeStamp(&self) -> u64
	{
		unsafe { rust_ibv_wc_read_completion_ts(self.0.pointer) }
	}
	
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_CVLAN
	#[inline(always)]
	pub fn strippedCvLan(&self) -> u16
	{
		unsafe { rust_ibv_wc_read_cvlan(self.0.pointer) }
	}
	
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_FLOW_TAG
	#[inline(always)]
	pub fn flowTag(&self) -> u32
	{
		unsafe { rust_ibv_wc_read_flow_tag(self.0.pointer) }
	}
}

