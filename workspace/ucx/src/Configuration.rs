// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct Configuration
{
	handle: *mut ucp_config_t,
}

impl Drop for Configuration
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ucp_config_release(self.handle) };
	}
}

impl PrintInformation for Configuration
{
	#[inline(always)]
	fn printInformationToStream(&self, stream: *mut FILE)
	{
		let printFlags = ucs_config_print_flags_t_UCS_CONFIG_PRINT_CONFIG | ucs_config_print_flags_t_UCS_CONFIG_PRINT_DOC | ucs_config_print_flags_t_UCS_CONFIG_PRINT_HEADER | ucs_config_print_flags_t_UCS_CONFIG_PRINT_HIDDEN;
		let title = CString::new("UCP Configuration").expect("Not a valid CStr");
		unsafe { ucp_config_print(self.handle, stream, title.as_ptr(), printFlags) };
	}
}

impl Configuration
{
	#[inline(always)]
	pub fn read(environmentVariablePrefix: &str) -> Self
	{
		let mut config_p = unsafe { uninitialized() };
		
		if environmentVariablePrefix.is_empty()
		{
			panic_on_error!(ucp_config_read, null(), null(), &mut config_p);
		}
		else
		{
			let prefix = CString::new(environmentVariablePrefix).expect("Not a valid CStr");
			panic_on_error!(ucp_config_read, prefix.as_ptr(), null(), &mut config_p);
		}
		
		Configuration
		{
			handle: config_p,
		}
	}
	
	/// See the static `ucp_config_table` in ucp_context.c for potential values of name and value
	#[inline(always)]
	pub fn modify(&self, name: &str, value: &str)
	{
		let name = CString::new(name).expect("Not a valid CStr");
		let value = CString::new(value).expect("Not a valid CStr");
		panic_on_error!(ucp_config_modify, self.handle, name.as_ptr(), value.as_ptr());
	}
	
	#[inline(always)]
	pub fn initialiseApplicationContext(&self, parameters: &ucp_params_t) -> ApplicationContext
	{
		let mut context = unsafe { uninitialized() };
		panic_on_error!(ucp_init_version, UCP_API_MAJOR, UCP_API_MINOR, parameters, self.handle, &mut context);
		ApplicationContext
		{
			handle: context,
		}
	}
}
