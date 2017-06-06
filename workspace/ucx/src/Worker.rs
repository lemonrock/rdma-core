// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Worker<'a>
{
	handle: ucp_worker_h,
	applicationContext: &'a ApplicationContext,
}

impl<'a> Drop for Worker<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_worker_destroy(self.handle) }
	}
}

impl<'a> Worker<'a>
{
	#[inline(always)]
	pub fn getFileDescriptorSuitableForEPoll(&self) -> RawFd
	{
		let mut fileDescriptorSuitableForEPoll = unsafe { uninitialized() };
		panic_on_error!(ucp_worker_get_efd, self.handle, &mut fileDescriptorSuitableForEPoll);
		fileDescriptorSuitableForEPoll
	}
	
	/// Returns an Err if internal logical returns UCS_ERR_IO_ERROR
	#[inline(always)]
	pub fn blockingWaitForAnyEvent(&self) -> Result<(), ()>
	{
		panic_on_error_with_clean_up!
		(
			status,
			{
				use ucs_status_t::*;
				match status
				{
					UCS_ERR_IO_ERROR => return Err(()),
					UCS_ERR_NO_MEMORY => panic!("Out of memory in 'ucp_worker_wait'"),
					_ => (),
				};
			},
			ucp_worker_wait,
			self.handle
		);
		Ok(())
	}
	
	/// Returns an Err if internal logical returns UCS_ERR_IO_ERROR
	#[inline(always)]
	pub fn blockingWaitForAMemoryEvent(&self, address: *mut c_void)
	{
		unsafe { ucp_worker_wait_mem(self.handle, address) }
	}
}
