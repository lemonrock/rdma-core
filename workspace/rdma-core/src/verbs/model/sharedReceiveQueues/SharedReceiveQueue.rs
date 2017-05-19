// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait SharedReceiveQueue
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_srq;
	
	#[doc(hidden)]
	#[inline(always)]
	fn settings(&mut self) -> &mut SharedReceiveQueueSettings;
	
	#[doc(hidden)]
	#[inline(always)]
	fn context(&self) -> &Context;
	
	#[doc(hidden)]
	#[inline(always)]
	fn isValidForProtectionDomain<'b>(&self, protectionDomain: &ProtectionDomain<'b>) -> bool;
	
	#[inline(always)]
	fn modifyLimit(&mut self, limit: u32)
	{
		debug_assert!(self.context().deviceHasCapability(ibv_device_cap_flags::IBV_DEVICE_SRQ_RESIZE), "device does not have IBV_DEVICE_SRQ_RESIZE capability");
		
		let mut attributes = ibv_srq_attr
		{
			max_wr: 0,
			max_sge: 0,
			srq_limit: limit,
		};
		
		panic_on_errno!(ibv_modify_srq, self.pointer(), &mut attributes, ibv_srq_attr_mask::IBV_SRQ_LIMIT as i32);
	}
	
	#[inline(always)]
	fn resize(&mut self, maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: u32)
	{
		debug_assert!(self.context().deviceHasCapability(ibv_device_cap_flags::IBV_DEVICE_SRQ_RESIZE), "device does not have IBV_DEVICE_SRQ_RESIZE capability");
		
		let mut attributes = ibv_srq_attr
		{
			max_wr: maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
			max_sge: 0,
			srq_limit: 0,
		};
		
		panic_on_errno!(ibv_modify_srq, self.pointer(), &mut attributes, ibv_srq_attr_mask::IBV_SRQ_MAX_WR as u32 as c_int);
		self.settings().maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue = attributes.max_wr;
	}
	
	#[inline(always)]
	fn currentLimit(&self) -> u32
	{
		let mut attributes = unsafe { uninitialized() };
		panic_on_errno!(ibv_query_srq, self.pointer(), &mut attributes);
		attributes.srq_limit
	}
	
	#[inline(always)]
	fn currentLimitIsUnset(&self) -> bool
	{
		self.currentLimit() == 0
	}
	
	#[inline(always)]
	fn number(&self) -> SharedReceiveQueueNumber
	{
		let mut number = unsafe { uninitialized() };
		panic_on_errno!(rust_ibv_get_srq_num, self.pointer(), &mut number);
		number
	}
	
	//pub fn rust_ibv_post_srq_recv(srq: *mut ibv_srq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int;
}
