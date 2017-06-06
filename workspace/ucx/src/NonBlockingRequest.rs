// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct NonBlockingRequest(ucs_status_ptr_t);

impl Drop for NonBlockingRequest
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if likely(self.0.UCS_PTR_IS_PTR())
		{
			unsafe { ucp_request_free(self.0) };
		}
	}
}

impl NonBlockingRequest
{
	#[inline(always)]
	pub fn isImmediateOkOrError(&self) -> Option<ucs_status_t>
	{
		if unlikely(self.0.UCS_PTR_IS_ERR())
		{
			Some(self.0.UCS_PTR_STATUS())
		}
		else
		{
			None
		}
	}
	
	#[inline(always)]
	pub fn currently(&self) -> Option<ucs_status_t>
	{
		if unlikely(self.0.UCS_PTR_IS_ERR())
		{
			Some(self.0.UCS_PTR_STATUS())
		}
		else
		{
			// This is only relevant for receive operations; we should consider having a separate implementation in that case
			let mut information = unsafe { uninitialized() };
			let result = unsafe { ucp_request_test(self.0, &mut information) };
			if unlikely(result == ucs_status_t::UCS_INPROGRESS)
			{
				None
			}
			else
			{
				Some(result)
			}
		}
	}
	
	#[inline(always)]
	pub fn cancel<'a>(self, workerThatOriginallyIssuedTheNonBlockingRequest: &Worker<'a>)
	{
		unsafe { ucp_request_cancel(workerThatOriginallyIssuedTheNonBlockingRequest.handle, self.0) }
	}
}
