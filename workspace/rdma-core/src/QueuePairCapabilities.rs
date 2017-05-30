// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
pub struct QueuePairCapabilities(ibv_qp_cap);

impl QueuePairCapabilities
{
	#[inline(always)]
	fn from(attributes: &ibv_qp_init_attr_ex) -> Self
	{
		Self(attributes.cap)
	}
	
	#[inline(always)]
	pub fn new
	(
		maximumNumberOfOutstandingWorkRequestsInTheSendQueue: u32,
		maximumNumberOfOutstandingWorkRequestsInTheReceiveQueue: u32,
		maximumNumberOfScatterGatherElementsInAWorkRequestInTheSendQueue: u32,
		maximumNumberOfScatterGatherElementsInAWorkRequestInTheReceiveQueue: u32,
		maximumNumberOfInlineDataBytesThatCanPostedInTheSendQueue: u32,
	) -> Self
	{
		Self
		(
			ibv_qp_cap
			{
				max_send_wr: maximumNumberOfOutstandingWorkRequestsInTheSendQueue,
				max_recv_wr: maximumNumberOfOutstandingWorkRequestsInTheReceiveQueue,
				max_send_sge: maximumNumberOfScatterGatherElementsInAWorkRequestInTheSendQueue,
				max_recv_sge: maximumNumberOfScatterGatherElementsInAWorkRequestInTheReceiveQueue,
				max_inline_data: maximumNumberOfInlineDataBytesThatCanPostedInTheSendQueue,
			}
		)
	}
	
	#[inline(always)]
	pub fn maximumNumberOfOutstandingWorkRequestsInTheSendQueue(&self) -> u32
	{
		self.0.max_send_wr
	}
	
	/// Ignored if the queue pair is associated with a shared receive queue
	#[inline(always)]
	pub fn maximumNumberOfOutstandingWorkRequestsInTheReceiveQueue(&self) -> u32
	{
		self.0.max_recv_wr
	}
	
	#[inline(always)]
	pub fn maximumNumberOfScatterGatherElementsInAWorkRequestInTheSendQueue(&self) -> u32
	{
		self.0.max_send_sge
	}
	
	/// Ignored if the queue pair is associated with a shared receive queue
	#[inline(always)]
	pub fn maximumNumberOfScatterGatherElementsInAWorkRequestInTheReceiveQueue(&self) -> u32
	{
		self.0.max_recv_sge
	}
	
	/// May be zero
	#[inline(always)]
	pub fn maximumNumberOfInlineDataBytesThatCanPostedInTheSendQueue(&self) -> u32
	{
		self.0.max_inline_data
	}
	
	#[inline(always)]
	pub fn as_ibv_qp_cap(&self) -> ibv_qp_cap
	{
		self.0
	}
}
