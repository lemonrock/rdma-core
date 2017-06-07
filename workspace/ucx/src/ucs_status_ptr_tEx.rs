// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ucs_status_ptr_tEx
{
	#[inline(always)]
	fn UCS_PTR_STATUS(self) -> ucs_status_t;
	
	#[inline(always)]
	fn UCS_PTR_IS_ERR(self) -> bool;
	
	#[inline(always)]
	fn UCS_PTR_IS_PTR(self) -> bool;
	
	#[inline(always)]
	fn UCS_STATUS_PTR(status: ucs_status_t) -> Self;
	
	#[inline(always)]
	fn ucsIsOk(self) -> bool;
}


use ::libc::intptr_t;
use ::libc::uintptr_t;
use ::std::mem::transmute;
impl ucs_status_ptr_tEx for ucs_status_ptr_t
{
	#[inline(always)]
	fn UCS_PTR_STATUS(self) -> ucs_status_t
	{
		unsafe { transmute(self as intptr_t as i8) }
	}
	
	#[inline(always)]
	fn UCS_PTR_IS_ERR(self) -> bool
	{
		(self as uintptr_t) >= (ucs_status_t_UCS_ERR_LAST as i8 as uintptr_t)
	}
	
	#[inline(always)]
	fn UCS_PTR_IS_PTR(self) -> bool
	{
		((self as uintptr_t) - 1) < ((ucs_status_t_UCS_ERR_LAST as i8 as uintptr_t) - 1)
	}
	
	#[inline(always)]
	fn UCS_STATUS_PTR(status: ucs_status_t) -> Self
	{
		status as intptr_t as *mut c_void
	}
	
	#[inline(always)]
	fn ucsIsOk(self) -> bool
	{
		debug_assert!(self.UCS_PTR_IS_ERR(), "?");
		debug_assert!(self as i8 == ucs_status_t_UCS_OK as i8, "?");
		
		self.is_null()
	}
}
