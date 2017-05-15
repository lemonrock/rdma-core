// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct SharedReceiveQueue<'a>
{
	pub(crate) pointer: *mut ibv_srq,
	pub(crate) settings: SharedReceiveQueueSettings,
	pub(crate) protectionDomain: &'a ProtectionDomain<'a>
}

impl<'a> Drop for SharedReceiveQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_srq, self.pointer);
	}
}

//pub fn rust_ibv_create_srq_ex(context: *mut ibv_context, srq_init_attr_ex: *mut ibv_srq_init_attr_ex) -> *mut ibv_srq;
//pub fn rust_ibv_get_srq_num(srq: *mut ibv_srq, srq_num: *mut u32) -> c_int;
//pub fn rust_ibv_post_srq_recv(srq: *mut ibv_srq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int;

impl<'a> SharedReceiveQueue<'a>
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
	
	#[inline(always)]
	pub fn number(&self) -> SharedReceiveQueueNumber
	{
		let mut number = unsafe { uninitialized() };
		panic_on_errno!(rust_ibv_get_srq_num, self.pointer, &mut number);
		number
	}
}
