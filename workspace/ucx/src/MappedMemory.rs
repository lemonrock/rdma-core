// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Mapped Memory is only needed for RMA and AMO (atomic operations)
#[derive(Debug)]
pub struct MappedMemory<'a>
{
	handle: ucp_mem_h,
	applicationContext: &'a ApplicationContext,
}

impl<'a> Drop for MappedMemory<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_error!(ucp_mem_unmap, self.applicationContext.handle, self.handle);
	}
}

impl<'a> QueryAttributes for MappedMemory<'a>
{
	type Attributes = MappedMemoryAttributes;
	
	#[inline(always)]
	fn queryAttributes(&self) -> Self::Attributes
	{
		use ucp_mem_attr_field::*;
		
		let mut attributes: ucp_mem_attr_t = unsafe { uninitialized() };
		attributes.field_mask = UCP_MEM_ATTR_FIELD_ADDRESS as u64 | UCP_MEM_ATTR_FIELD_LENGTH as u64;
		
		panic_on_error!(ucp_mem_query, self.handle, &mut attributes);
		MappedMemoryAttributes(attributes)
	}
}

impl<'a> MappedMemory<'a>
{
	// Need to look at ucp_mem_advise_params_field
	#[inline(always)]
	pub fn advise(&self, parameters: &mut ucp_mem_advise_params_t)
	{
		panic_on_error!(ucp_mem_advise, self.applicationContext.handle, self.handle, parameters);
	}
	
	#[inline(always)]
	pub fn packRemoteMemoryAccessKey<'b>(&'b self) -> RemoteMemoryAccessKeyBuffer<'a, 'b>
	{
		let mut buffer = unsafe { uninitialized() };
		let mut size = unsafe { uninitialized() };
		panic_on_error!(ucp_rkey_pack, self.applicationContext.handle, self.handle, &mut buffer, &mut size);
		RemoteMemoryAccessKeyBuffer
		{
			address: buffer,
			length: size,
			mappedMemory: self,
		}
	}
}
