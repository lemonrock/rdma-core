// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct ApplicationContext
{
	handle: ucp_context_h,
}

impl Drop for ApplicationContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_cleanup(self.handle) };
	}
}

impl ApplicationContext
{
	#[inline(always)]
	pub fn attributes(&self) -> ucp_context_attr_t
	{
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ucp_context_query, self.handle, &mut attributes);
		attributes
	}
	
	#[inline(always)]
	pub fn printInformationToStandardError(&self)
	{
		unsafe { ucp_context_print_info(self.handle, stderr as *mut FILE) };
	}
	
	#[inline(always)]
	pub fn mapMemory<'a>(&'a self, address: *mut c_void, length: usize) -> MappedMemory<'a>
	{
		debug_assert!(!address.is_null(), "address is null");
		debug_assert!(length != 0, "length is zero");
		
		// Use ucp_mem_map_params_field::UCP_MEM_MAP_PARAM_FIELD_FLAGS as u64 and flags to force memory with allocation, too
		
		let parameters = ucp_mem_map_params_t
		{
			field_mask: ucp_mem_map_params_field::UCP_MEM_MAP_PARAM_FIELD_ADDRESS as u64 | ucp_mem_map_params_field::UCP_MEM_MAP_PARAM_FIELD_LENGTH as u64,
			address: address,
			length: length,
			flags: 0,
		};
		
		let mut memh = unsafe { uninitialized() };
		panic_on_error!(ucp_mem_map, self.handle, &parameters, &mut memh);
		MappedMemory
		{
			applicationContext: self,
			handle: memh,
		}
	}
}
