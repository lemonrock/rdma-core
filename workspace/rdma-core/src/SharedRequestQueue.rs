// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct SharedRequestQueue<'a>
{
	pointer: *mut ibv_srq,
	settings: SharedRequestQueueSettings,
	protectionDomain: &'a ProtectionDomain<'a>
}

impl<'a> Drop for SharedRequestQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_srq, self.pointer);
	}
}

impl<'a> SharedRequestQueue<'a>
{
	#[inline(always)]
	pub fn modifyLimit(&mut self, limit: u32)
	{
		debug_assert!(self.protectionDomain.context.deviceHasCapability(ibv_device_cap_flags::IBV_DEVICE_SRQ_RESIZE), "device does not have IBV_DEVICE_SRQ_RESIZE capability");
		
		let mut attributes = ibv_srq_attr
		{
			max_wr: 0,
			max_sge: 0,
			srq_limit: limit,
		};
		
		panic_on_errno!(ibv_modify_srq, self.pointer, &mut attributes, ibv_srq_attr_mask::IBV_SRQ_LIMIT as i32);
	}
	
	#[inline(always)]
	pub fn resize(&mut self, maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue: u32)
	{
		debug_assert!(self.protectionDomain.context.deviceHasCapability(ibv_device_cap_flags::IBV_DEVICE_SRQ_RESIZE), "device does not have IBV_DEVICE_SRQ_RESIZE capability");
		
		let mut attributes = ibv_srq_attr
		{
			max_wr: maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue,
			max_sge: 0,
			srq_limit: 0,
		};
		
		panic_on_errno!(ibv_modify_srq, self.pointer, &mut attributes, ibv_srq_attr_mask::IBV_SRQ_MAX_WR as u32 as c_int);
		self.settings.maximumNumberOfOutstandingWorkRequestsInInTheSharedRequestQueue = attributes.max_wr;
	}
	
	#[inline(always)]
	pub fn currentLimit(&self) -> u32
	{
		let mut attributes = unsafe { uninitialized() };
		panic_on_errno!(ibv_query_srq, self.pointer, &mut attributes);
		attributes.srq_limit
	}
	
	#[inline(always)]
	pub fn currentLimitIsUnset(&self) -> bool
	{
		self.currentLimit() == 0
	}
}
