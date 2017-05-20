// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_deviceEx
{
	#[inline(always)]
	fn name<'a>(self) -> &'a CStr;
	
	#[inline(always)]
	fn deviceName<'a>(self) -> &'a CStr;
	
	/// This is believed to be a SysFS folder path
	/// The underlying implementation logic uses an UTF-8 CStr, which is incorrect
	#[inline(always)]
	fn infinibandDevicePath<'a>(self) -> &'a Path;
	
	/// This is believed to be a SysFS folder path
	/// It isn't used in the libibverbs sources for anything, and it's possible it is invalid
	#[inline(always)]
	fn devicePath<'a>(self) -> &'a Path;
	
	#[inline(always)]
	fn nodeType<'a>(self) -> ibv_node_type;
	
	#[inline(always)]
	fn transportType<'a>(self) -> ibv_transport_type;
}

impl ibv_deviceEx for *mut ibv_device
{
	#[inline(always)]
	fn name<'a>(self) -> &'a CStr
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let string = unsafe { (*self).name };
		cStrFromFixedLengthString(&string as *const c_char, 64)
	}
	
	#[inline(always)]
	fn deviceName<'a>(self) -> &'a CStr
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let string = unsafe { (*self).dev_name };
		cStrFromFixedLengthString(&string as *const c_char, 64)
	}
	
	#[inline(always)]
	fn infinibandDevicePath<'a>(self) -> &'a Path
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let string = unsafe { (*self).ibdev_path };
		let length = unsafe { strnlen(&string as *const c_char, 256) };
		Path::new(OsStr::from_bytes(unsafe { transmute(&string[0 .. length]) }))
	}
	
	#[inline(always)]
	fn devicePath<'a>(self) -> &'a Path
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let string = unsafe { (*self).dev_path };
		let length = unsafe { strnlen(&string as *const c_char, 256) };
		Path::new(OsStr::from_bytes(unsafe { transmute(&string[0 .. length]) }))
	}
	
	#[inline(always)]
	fn nodeType<'a>(self) -> ibv_node_type
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).node_type }
	}
	
	#[inline(always)]
	fn transportType<'a>(self) -> ibv_transport_type
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).transport_type }
	}
}

