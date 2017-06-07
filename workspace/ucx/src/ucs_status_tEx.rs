// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ucs_status_tEx
{
	#[inline(always)]
	fn UCS_IS_LINK_ERROR(self) -> bool;
	
	#[inline(always)]
	fn UCS_IS_ENDPOINT_ERROR(self) -> bool;
	
	#[inline(always)]
	fn string(self) -> &'static CStr;
	
	#[inline(always)]
	fn isOk(self) -> bool;
}

impl ucs_status_tEx for ucs_status_t
{
	#[inline(always)]
	fn UCS_IS_LINK_ERROR(self) -> bool
	{
		let code = self as i8;
		code <= ucs_status_t_UCS_ERR_FIRST_LINK_FAILURE as i8 && code >= ucs_status_t_UCS_ERR_LAST_LINK_FAILURE as i8
	}
	
	#[inline(always)]
	fn UCS_IS_ENDPOINT_ERROR(self) -> bool
	{
		let code = self as i8;
		code <= ucs_status_t_UCS_ERR_FIRST_ENDPOINT_FAILURE as i8 && code >= ucs_status_t_UCS_ERR_LAST_ENDPOINT_FAILURE as i8
	}
	
	#[inline(always)]
	fn string(self) -> &'static CStr
	{
		unsafe { CStr::from_ptr(ucs_status_string(self)) }
	}
	
	#[inline(always)]
	fn isOk(self) -> bool
	{
		self == ucs_status_t_UCS_OK
	}
}
