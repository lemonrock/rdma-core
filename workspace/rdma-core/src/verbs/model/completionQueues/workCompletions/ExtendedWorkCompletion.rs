// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ExtendedWorkCompletion<'a, E: ExtendedCompletionQueue<'a>>(pub(crate) &'a E)
where E: 'a;

impl<'a, E: ExtendedCompletionQueue<'a>> WorkCompletion<'a> for ExtendedWorkCompletion<'a, E>
{
	type ValidWorkCompletion = ExtendedValidWorkCompletion<'a, E>;
	
	#[inline(always)]
	fn workRequestIdentifier(&self) -> WorkRequestIdentifier
	{
		self.pointer().workRequest()
	}
	
	/// Not present if the extended completion queue was not created with IBV_WC_EX_WITH_QP_NUM
	#[inline(always)]
	fn receiveWorkRequestLocalQueuePairNumberForSharedReceiveQueue(&self) -> QueuePairNumber
	{
		self.pointer().ibv_wc_read_qp_num()
	}
	
	#[inline(always)]
	fn workRequestError(self) -> Result<Self::ValidWorkCompletion, WorkRequestError>
	{
		let status = self.pointer().status();
		
		if likely(status == ibv_wc_status::IBV_WC_SUCCESS)
		{
			Ok(ExtendedValidWorkCompletion(self.0))
		}
		else
		{
			Err(WorkRequestError
			{
				status: status,
				vendorErrorCode: self.pointer().ibv_wc_read_vendor_err(),
			})
		}
	}
}

impl<'a, E: ExtendedCompletionQueue<'a>> ExtendedWorkCompletion<'a, E>
{
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq_ex
	{
		self.0.extendedPointer()
	}
}
