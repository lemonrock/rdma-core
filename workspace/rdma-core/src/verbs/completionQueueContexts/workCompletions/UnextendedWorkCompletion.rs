// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct UnextendedWorkCompletion(pub(crate) ibv_wc);

impl WorkCompletion for UnextendedWorkCompletion
{
	type ValidWorkCompletion = UnextendedValidWorkCompletion;
	
	#[inline(always)]
	fn workRequestIdentifier(&self) -> WorkRequestIdentifier
	{
		self.0.wr_id
	}
	
	#[inline(always)]
	fn receiveWorkRequestLocalQueuePairNumberForSharedReceiveQueue(&self) -> QueuePairNumber
	{
		self.0.qp_num
	}
	
	#[inline(always)]
	fn workRequestError(self) -> Result<Self::ValidWorkCompletion, WorkRequestError>
	{
		let status = self.0.status;
		if likely(status == ibv_wc_status::IBV_WC_SUCCESS)
		{
			Ok(UnextendedValidWorkCompletion(self))
		}
		else
		{
			Err(WorkRequestError
			{
				status: status,
				vendorErrorCode: self.0.vendor_err
			})
		}
	}
}
