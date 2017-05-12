// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct WorkCompletion(ibv_wc);

impl WorkCompletion
{
	#[inline(always)]
	pub fn workRequestIdentifier(&self) -> WorkRequestIdentifier
	{
		self.0.wr_id
	}
	
	#[inline(always)]
	pub fn receiveWorkRequestLocalQueuePairNumberForSharedReceiveQueue(&self) -> QueuePairNumber
	{
		self.0.qp_num
	}
	
	#[inline(always)]
	pub fn workRequestError<'a>(&'a self) -> Result<ValidWorkCompletion<'a>, WorkRequestError>
	{
		if likely(self.0.status == ibv_wc_status::IBV_WC_SUCCESS)
		{
			Ok(ValidWorkCompletion
			{
				workCompletion: self
			})
		}
		else
		{
			Err(WorkRequestError
			{
				status: self.0.status,
				vendorErrorCode: self.0.vendor_err
			})
		}
	}
}
