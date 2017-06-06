// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct WorkerAddressHandle<'a, 'b>
where 'a: 'b
{
	handle: *mut ucp_address_t,
	length: usize,
	worker: &'b Worker<'a>,
}

impl<'a, 'b> Drop for WorkerAddressHandle<'a, 'b>
where 'a: 'b
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_worker_release_address(self.worker.handle, self.handle) }
	}
}

