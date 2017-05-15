// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct SharedRequestQueue<'a>
{
	pub(crate) pointer: *mut ibv_srq,
	pub(crate) settings: SharedRequestQueueSettings,
	pub(crate) protectionDomain: &'a ProtectionDomain<'a>
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

/*

static inline struct ibv_srq* ibv_create_srq_ex(struct ibv_context* context, struct ibv_srq_init_attr_ex* srq_init_attr_ex)
{
	struct verbs_context* vctx;
	uint32_t mask = srq_init_attr_ex->comp_mask;

	if (!(mask & ~(IBV_SRQ_INIT_ATTR_PD | IBV_SRQ_INIT_ATTR_TYPE)) && (mask & IBV_SRQ_INIT_ATTR_PD) && (!(mask & IBV_SRQ_INIT_ATTR_TYPE) || (srq_init_attr_ex->srq_type == IBV_SRQT_BASIC)))
	{
		return ibv_create_srq(srq_init_attr_ex->pd, (struct ibv_srq_init_attr*)srq_init_attr_ex);
	}
	
	vctx = verbs_get_ctx_op(context, create_srq_ex);
	if (!vctx)
	{
		errno = ENOSYS;
		return NULL;
	}

	return vctx->create_srq_ex(context, srq_init_attr_ex);
}

static inline int ibv_get_srq_num(struct ibv_srq* srq, uint32_t* srq_num)
{
	struct verbs_context* vctx = verbs_get_ctx_op(srq->context, get_srq_num);

	if (!vctx)
	{
		return ENOSYS;
	}
	
	return vctx->get_srq_num(srq, srq_num);
}

static inline int ibv_post_srq_recv(struct ibv_srq* srq, struct ibv_recv_wr* recv_wr, struct ibv_recv_wr** bad_recv_wr)
{
	return srq->context->ops.post_srq_recv(srq, recv_wr, bad_recv_wr);
}

*/
