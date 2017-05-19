// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ProtectionDomain<'a>
{
	pointer: *mut ibv_pd,
	context: &'a Context,
}

impl<'a> Drop for ProtectionDomain<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.pointer.destroy();
	}
}

impl<'a> ProtectionDomain<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_pd, context: &'a Context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			context: context,
		}
	}
	
	#[inline(always)]
	pub fn registerMemoryRegion(&'a self, address: *mut c_void, length: usize, access: &MemoryRegionAccess) -> MemoryRegion<'a>
	{
		debug_assert!(!address.is_null(), "address can not be null");
		debug_assert!(length != 0, "length can not be zero");
		debug_assert!(length as u64 <= self.context.attributes().max_mr_size, "length '{}' exceeds device maximum '{}'", length, self.context.attributes().max_mr_size);
		
		let pointer = panic_on_null!(ibv_reg_mr, self.pointer, address, length, access.as_c_int());
		MemoryRegion
		{
			pointer: pointer,
			lifetime: PhantomData,
		}
	}
	
	// See: https://www.mankier.com/3/ibv_bind_mw
	// Needs a queue pair, a memory region, a WorkRequestIdentifier, memory address & length, access flags, send flags
	#[inline(always)]
	pub fn allocateType1MemoryWindow(&'a self) -> MemoryWindow
	{
		self.allocateMemoryWindow(ibv_mw_type::IBV_MW_TYPE_1);
		unimplemented!();
	}
	
	#[inline(always)]
	fn allocateMemoryWindow(&'a self, memoryWindowType: ibv_mw_type) -> MemoryWindow
	{
		// ibv_post_send should be called for type 2
		// ibv_bind_mw must be called for type 1 : pub fn rust_ibv_bind_mw(qp: *mut ibv_qp, mw: *mut ibv_mw, mw_bind: *mut ibv_mw_bind) -> c_int;
		
		let pointer = panic_on_null!(rust_ibv_alloc_mw, self.pointer, memoryWindowType);
		MemoryWindow
		{
			pointer: pointer,
			protectionDomain: self,
		}
	}
	
	// TODO: An extended shared recv queue also has a completion queue, and is only for valid for XRC queue pairs I suspect
	
	
//	#[inline(always)]
//	pub fn createUnextendedQueuePair<SendC: CompletionQueue, ReceiveC: CompletionQueue, SharedReceive: SharedReceiveQueue>(&'a self, sendCompletionQueue: &'a SendC, receiveCompletionQueue: &'a ReceiveC, sharedReceiveQueue: Option<&'a SharedReceive>, capabilities: ibv_qp_cap, eachWorkRequestSubmittedToTheSendCompletionQueueGeneratesACompletionEntry: bool) -> UnextendedQueuePair<'a, SendC, ReceiveC, SharedReceive>
//	{
//		let context = self.context;
//		assert!(sendCompletionQueue.isValidForContext(context), "sendCompletionQueue is not valid for this protection domain's context");
//		assert!(receiveCompletionQueue.isValidForContext(context), "receiveCompletionQueue is not valid for this protection domain's context");
//
//		let mut attributes = ibv_qp_init_attr
//		{
//			qp_context: null_mut(),
//			send_cq: sendCompletionQueue.pointer(),
//			recv_cq: receiveCompletionQueue.pointer(),
//			srq: match sharedReceiveQueue
//			{
//				None => null_mut(),
//				Some(sharedReceiveQueue) =>
//				{
//					assert!(sharedReceiveQueue.isValidForProtectionDomain(self), "shared receive queue is not valid for this protection domain");
//
//					sharedReceiveQueue.pointer()
//				},
//			},
//			cap: capabilities,
//			qp_type: ibv_qp_type::IBV_QPT_RC,
//			sq_sig_all: if unlikely(eachWorkRequestSubmittedToTheSendCompletionQueueGeneratesACompletionEntry)
//			{
//				1
//			}
//			else
//			{
//				0
//			},
//		};
//
//		/*
//			IBV_QPT_RC = 2,
//			IBV_QPT_UC = 3,
//			IBV_QPT_UD = 4,
//			IBV_QPT_RAW_PACKET = 8,
//			IBV_QPT_XRC_SEND = 9,
//			IBV_QPT_XRC_RECV = 10,
//		*/
//
//		let pointer = panic_on_null!(ibv_create_qp, self.pointer, &mut attributes);
//		UnextendedQueuePair::new(pointer, attributes.cap, (self, sendCompletionQueue, receiveCompletionQueue, sharedReceiveQueue))
//
//		/*
//	pub fn rust_ibv_create_qp_ex(context: *mut ibv_context, qp_init_attr_ex: *mut ibv_qp_init_attr_ex) -> *mut ibv_qp;
//	pub fn rust_ibv_open_qp(context: *mut ibv_context, qp_open_attr: *mut ibv_qp_open_attr) -> *mut ibv_qp;
//
//
//		*/
//	}
}
