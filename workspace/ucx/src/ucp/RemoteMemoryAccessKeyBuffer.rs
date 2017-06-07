// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct RemoteMemoryAccessKeyBuffer<'a, 'b>
where 'a: 'b
{
	address: *mut c_void,
	length: usize,
	mappedMemory: &'b MappedMemory<'a>
}

impl<'a, 'b> Drop for RemoteMemoryAccessKeyBuffer<'a, 'b>
where 'a: 'b
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_rkey_buffer_release(self.address) }
	}
}

impl<'a, 'b> RemoteMemoryAccessKeyBuffer<'a, 'b>
where 'a: 'b
{
	#[inline(always)]
	pub fn address(&self) -> *mut c_void
	{
		self.address
	}
	
	#[inline(always)]
	pub fn length(&self) -> usize
	{
		self.length
	}
}
