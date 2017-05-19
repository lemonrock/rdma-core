// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[derive(Debug)]
pub struct Device<'a>
{
	pointer: *mut ibv_device,
	parent: PhantomData<&'a mut DeviceListIterator<'a>>
}

impl<'a> Device<'a>
{
	#[inline(always)]
	pub fn name(&self) -> &'a CStr
	{
		unsafe { CStr::from_ptr(ibv_get_device_name(self.pointer)) }
	}
	
	#[inline(always)]
	pub fn nodeGuid(&self) -> GUID
	{
		GUID(unsafe { ibv_get_device_guid(self.pointer) })
	}
	
	#[inline(always)]
	pub fn openContext(self) -> Context
	{
		Context::new(panic_on_null!(ibv_open_device, self.pointer))
	}
}
