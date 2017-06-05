// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct ApplicationContext
{
	context_p: ucp_context_h,
}

impl Drop for ApplicationContext
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_cleanup(self.context_p) };
	}
}

impl ApplicationContext
{
	#[inline(always)]
	pub fn attributes(&self) -> ucp_context_attr_t
	{
		let mut attributes = unsafe { uninitialized() };
		panic_on_error!(ucp_context_query, self.context_p, &mut attributes);
		attributes
	}
	
	#[inline(always)]
	pub fn printInformationToStandardError(&self)
	{
		unsafe { ucp_context_print_info(self.context_p, stderr as *mut FILE) };
	}
}
