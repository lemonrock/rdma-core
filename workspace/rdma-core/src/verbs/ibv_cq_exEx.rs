// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_cq_exEx: CompletionQueuePointer
{
	#[inline(always)]
	fn ibv_cq_ex_to_cq(self) -> *mut ibv_cq;
	
	#[inline(always)]
	fn ibv_start_poll(self, attr: *mut ibv_poll_cq_attr) -> c_int;
	
	#[inline(always)]
	fn ibv_next_poll(self) -> c_int;
	
	#[inline(always)]
	fn ibv_end_poll(self);
	
	#[inline(always)]
	fn ibv_wc_read_opcode(self) -> ibv_wc_opcode;
	
	#[inline(always)]
	fn ibv_wc_read_vendor_err(self) -> uint32_t;
	
	/// The number of bytes transferred. Relevant if the Receive Queue for incoming Send or RDMA Write with immediate operations. This value doesn't include the length of the immediate data, if such exists. Relevant in the Send Queue for RDMA Read and Atomic operations.
	/// For the Receive Queue of a UD QP that is not associated with an SRQ or for an SRQ that is associated with a UD QP this value equals to the payload of the message plus the 40 bytes reserved for the GRH.
	/// The number of bytes transferred is the payload of the message plus the 40 bytes reserved for the GRH, whether or not the GRH is present
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_BYTE_LEN
	#[inline(always)]
	fn ibv_wc_read_byte_len(self) -> uint32_t;
	
	/// Only relevant for Unreliable Datagrams
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_IMM
	#[inline(always)]
	fn ibv_wc_read_imm_data(self) -> uint32_t;
	
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_QP_NUM
	#[inline(always)]
	fn ibv_wc_read_qp_num(self) -> QueuePairNumber;
	
	/// Only relevant for Unreliable Datagrams
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_SRC_QP
	#[inline(always)]
	fn ibv_wc_read_src_qp(self) -> QueuePairNumber;
	
	#[inline(always)]
	fn ibv_wc_read_wc_flags(self) -> c_int;
	
	/// Only relevant for Unreliable Datagrams
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_SLID
	#[inline(always)]
	fn ibv_wc_read_slid(self) -> LocalIdentifier;
	
	/// Only relevant for Unreliable Datagrams
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_SL
	#[inline(always)]
	fn ibv_wc_read_sl(self) -> ServiceLevel;
	
	/// Only relevant for Unreliable Datagrams
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_DLID_PATH_BITS
	#[inline(always)]
	fn ibv_wc_read_dlid_path_bits(self) -> LocalIdentifierPath;
	
	/// Not all devices (contexts, verbs) support this
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_COMPLETION_TIMESTAMP
	#[inline(always)]
	fn ibv_wc_read_completion_ts(self) -> uint64_t;
	
	/// Not all devices (contexts, verbs) support this
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_CVLAN
	#[inline(always)]
	fn ibv_wc_read_cvlan(self) -> uint16_t;
	
	/// Not all devices (contexts, verbs) support this
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_FLOW_TAG
	#[inline(always)]
	fn ibv_wc_read_flow_tag(self) -> uint32_t;
	
	#[inline(always)]
	fn workRequest(self) -> WorkRequestIdentifier;
	
	#[inline(always)]
	fn status(self) -> ibv_wc_status;
}

impl ibv_cq_exEx for *mut ibv_cq_ex
{
	#[inline(always)]
	fn ibv_cq_ex_to_cq(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self as *mut ibv_cq
	}
	
	#[inline(always)]
	fn ibv_start_poll(self, attr: *mut ibv_poll_cq_attr) -> c_int
	{
		debug_assert!(!self.is_null(), "self is null");
		debug_assert!(!attr.is_null(), "attr is null");
		
		unsafe { (*self).start_poll.unwrap()(self, attr) }
	}
	
	#[inline(always)]
	fn ibv_next_poll(self) -> c_int
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).next_poll.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_end_poll(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).end_poll.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_opcode(self) -> ibv_wc_opcode
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_opcode.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_vendor_err(self) -> uint32_t
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_vendor_err.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_byte_len(self) -> uint32_t
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_byte_len.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_imm_data(self) -> uint32_t
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_imm_data.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_qp_num(self) -> QueuePairNumber
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_qp_num.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_src_qp(self) -> QueuePairNumber
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_src_qp.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_wc_flags(self) -> c_int
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_wc_flags.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_slid(self) -> LocalIdentifier
	{
		debug_assert!(!self.is_null(), "self is null");
		
		(unsafe { (*self).read_slid.unwrap()(self) }) as u16
	}
	
	#[inline(always)]
	fn ibv_wc_read_sl(self) -> ServiceLevel
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { transmute((*self).read_sl.unwrap()(self)) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_dlid_path_bits(self) -> LocalIdentifierPath
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_dlid_path_bits.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_completion_ts(self) -> uint64_t
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_completion_ts.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_cvlan(self) -> uint16_t
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_cvlan.unwrap()(self) }
	}
	
	#[inline(always)]
	fn ibv_wc_read_flow_tag(self) -> uint32_t
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).read_flow_tag.unwrap()(self) }
	}
	
	#[inline(always)]
	fn workRequest(self) -> WorkRequestIdentifier
	{
		unsafe { (*self).wr_id }
	}
	
	#[inline(always)]
	fn status(self) -> ibv_wc_status
	{
		unsafe { (*self).status }
	}
}
