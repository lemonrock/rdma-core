// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler: EndPointErrorHandler>
where 'a: 'b, 'b: 'c, ErrorHandler: 'c
{
	handle: ucp_rkey_h,
	endPoint: &'c EndPoint<'a, 'b, ErrorHandler>
}

impl<'a, 'b, 'c, ErrorHandler: EndPointErrorHandler> Drop for RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler>
where 'a: 'b, 'b: 'c, ErrorHandler: 'c
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_rkey_destroy(self.handle) }
	}
}

impl<'a, 'b, 'c, ErrorHandler: EndPointErrorHandler> RemoteMemoryAccessKey<'a, 'b, 'c, ErrorHandler>
where 'a: 'b, 'b: 'c, ErrorHandler: 'c
{
	/// Presumably we need to have received a message telling us the remoteAddress... perhaps at the same time we get the rkey
	#[inline(always)]
	pub fn localMemoryAddressThatCanBeUsedToDirectLoadsAndStoresInRemoteMemory(&self, remoteAddress: *mut c_void) -> *mut c_void
	{
		let mut localAddress = unsafe { uninitialized() };
		panic_on_error!(ucp_rmem_ptr, self.endPoint.handle, remoteAddress, self.handle, &mut localAddress);
		localAddress
	}
}
