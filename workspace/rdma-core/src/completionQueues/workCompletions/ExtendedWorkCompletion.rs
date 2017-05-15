// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ExtendedWorkCompletion<'a>(pub(crate) &'a ExtendedCompletionQueue<'a>);

impl<'a> WorkCompletion<'a> for ExtendedWorkCompletion<'a>
{
	type ValidWorkCompletion = ExtendedValidWorkCompletion<'a>;
	
	#[inline(always)]
	fn workRequestIdentifier(&self) -> WorkRequestIdentifier
	{
		self.data().wr_id
	}
	
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_QP_NUM
	#[inline(always)]
	fn receiveWorkRequestLocalQueuePairNumberForSharedReceiveQueue(&self) -> QueuePairNumber
	{
		unsafe { rust_ibv_wc_read_qp_num(self.pointer()) }
	}
	
	#[inline(always)]
	fn workRequestError(self) -> Result<Self::ValidWorkCompletion, WorkRequestError>
	{
		let status = self.data().status;
		
		if likely(status == ibv_wc_status::IBV_WC_SUCCESS)
		{
			Ok(ExtendedValidWorkCompletion(self.0))
		}
		else
		{
			Err(WorkRequestError
			{
				status: status,
				vendorErrorCode: unsafe { rust_ibv_wc_read_vendor_err(self.pointer()) },
			})
		}
	}
}

impl<'a> ExtendedWorkCompletion<'a>
{
	#[inline(always)]
	fn data(&self) -> ibv_cq_ex
	{
		unsafe { *self.pointer() }
	}
	
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq_ex
	{
		self.0.pointer
	}
}
