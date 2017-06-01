// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct EventChannelVerbMapEntry<'a>
{
	protectionDomain: *mut ibv_pd,
	extendedReliableConnectionDomain: *mut ibv_xrcd,
	completionChannel: Box<EPollContextChoice<'a>>,
}

impl<'a> Drop for EventChannelVerbMapEntry<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.extendedReliableConnectionDomain.destroy();
		self.protectionDomain.destroy();
	}
}

impl<'a> VerbMapEntry<'a> for EventChannelVerbMapEntry<'a>
{
	type ConstructionParameters = EPoll<EPollContextChoice<'a>>;
	
	#[inline(always)]
	fn create(constructionParameters: &'a Self::ConstructionParameters, verbs: *mut ibv_context) -> Self
	{
		Self
		{
			protectionDomain: verbs.allocateProtectionDomain(),
			extendedReliableConnectionDomain: verbs.createExtendedReliableConnectionDomainWithoutInode(),
			completionChannel: CompletionChannel::new(constructionParameters, verbs),
		}
	}
}

impl<'a> EventChannelVerbMapEntry<'a>
{
	#[inline(always)]
	pub fn createCompletionQueue(&mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void)
	{
		match *self.completionChannel
		{
			EPollContextChoice::CompletionChannel(ref mut context) => context.createCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext),
			_ => (),
		}
	}
	
	#[inline(always)]
	pub fn createExtendedCompletionQueue(&mut self, atLeastThisNumberOfCompletionQueueEvents: u32, completionQueueContext: *mut c_void, workCompletionFlags: ibv_create_cq_wc_flags, lockLessButNotThreadSafe: bool)
	{
		match *self.completionChannel
		{
			EPollContextChoice::CompletionChannel(ref mut context) => context.createExtendedCompletionQueue(atLeastThisNumberOfCompletionQueueEvents, completionQueueContext, workCompletionFlags, lockLessButNotThreadSafe),
			_ => (),
		}
	}
	
	// TODO: Must call ibv_destroy_qp()
	/// maximumTsoHeaderSize is only supported By Mellanox mlx5 drivers
	/// I suspect scatterFrameCheckSequence and cvLanStripping is likewise only support by mlx5
	#[inline(always)]
	pub fn createReliablyConnectedExtendedQueuePair(&mut self, queuePairContext: *mut c_void, sendCompletionQueue: *mut ibv_cq, receiveCompletionQueue: *mut ibv_cq, eachWorkRequestSubmittedToTheSendQueueGeneratesACompletionEntry: bool, queuePairCapabilities: &QueuePairCapabilities, scatterFrameCheckSequence: bool, cvLanStripping: bool, maximumTsoHeaderSize: u16) -> (*mut ibv_qp, QueuePairCapabilities)
	{
		debug_assert!(!sendCompletionQueue.is_null(), "sendCompletionQueue is null");
		debug_assert!(!receiveCompletionQueue.is_null(), "receiveCompletionQueue is null");
		
		let verbs = self.protectionDomain.verbs();
		
		#[allow(dead_code)] const IBV_QP_CREATE_BLOCK_SELF_MCAST_LB: u32 = 2;
		const IBV_QP_CREATE_SCATTER_FCS: u32 = 256;
		const IBV_QP_CREATE_CVLAN_STRIPPING: u32 = 512;
		
		let mut creationFlags = if unlikely(scatterFrameCheckSequence)
		{
			IBV_QP_CREATE_SCATTER_FCS
		}
		else
		{
			0
		};
		if unlikely(cvLanStripping)
		{
			creationFlags |= IBV_QP_CREATE_CVLAN_STRIPPING;
		}
		
		const IBV_QP_INIT_ATTR_PD: u32 = 1;
		const IBV_QP_INIT_ATTR_XRCD: u32 = 2;
		const IBV_QP_INIT_ATTR_CREATE_FLAGS: u32 = 4;
		const IBV_QP_INIT_ATTR_MAX_TSO_HEADER: u32 = 8;
		#[allow(dead_code)] const IBV_QP_INIT_ATTR_IND_TABLE: u32 = 16;
		#[allow(dead_code)] const IBV_QP_INIT_ATTR_RX_HASH: u32 = 32;
		
		let mut compMask = IBV_QP_INIT_ATTR_PD | IBV_QP_INIT_ATTR_XRCD | IBV_QP_INIT_ATTR_CREATE_FLAGS;
		if maximumTsoHeaderSize > 0
		{
			compMask |= IBV_QP_INIT_ATTR_MAX_TSO_HEADER;
		}
		
		let mut attributes = ibv_qp_init_attr_ex
		{
			qp_context: queuePairContext,
			send_cq: sendCompletionQueue,
			recv_cq: receiveCompletionQueue,
			srq: null_mut(),
			cap: queuePairCapabilities.as_ibv_qp_cap(),
			sq_sig_all: if unlikely(eachWorkRequestSubmittedToTheSendQueueGeneratesACompletionEntry)
			{
				1
			}
			else
			{
				0
			},
			qp_type: ibv_qp_type::IBV_QPT_RC, // also IBV_QPT_XRC_SEND and IBV_QPT_XRC_RECV
			comp_mask: compMask,
			pd: self.protectionDomain,
			xrcd: self.extendedReliableConnectionDomain,
			create_flags: creationFlags,
			max_tso_header: maximumTsoHeaderSize,
			rwq_ind_tbl: null_mut(),
			rx_hash_conf: ibv_rx_hash_conf
			{
				rx_hash_function: 0,
				rx_hash_key_len: 0,
				rx_hash_key: null_mut(),
				rx_hash_fields_mask: 0,
			},
		};
		
		let queuePairPointer = panic_on_null!(rust_ibv_create_qp_ex, verbs, &mut attributes);
		(queuePairPointer, QueuePairCapabilities::from(&attributes))
	}
}
