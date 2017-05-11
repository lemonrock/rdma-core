// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug)]
pub struct DeviceListIterator<'a>
{
	list: *mut *mut ibv_device,
	size: usize,
	next: *mut ibv_device,
	lifetime: PhantomData<&'a ibv_device>,
}

impl<'a> Drop for DeviceListIterator<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		unsafe { ibv_free_device_list(self.list) }
	}
}

impl<'a> Iterator for DeviceListIterator<'a>
{
	type Item = Device<'a>;
	
	#[inline(always)]
	fn next(&mut self) -> Option<Self::Item>
	{
		let next = self.next;
		if unlikely(next.is_null())
		{
			return None;
		}
		self.next = unsafe { next.offset(1) };
		Some
		(
			Device
			{
				pointer: next,
				parent: PhantomData,
			}
		)
	}
}

impl<'a> DeviceListIterator<'a>
{
	#[inline(always)]
	pub fn devices() -> Self
	{
		let mut size = 0;
		let list = unsafe { ibv_get_device_list(&mut size) };
		if unlikely(list.is_null())
		{
			let errno = errno();
			match errno.0
			{
				E::EPERM => panic!("Permission denied"),
				E::ENOSYS => panic!("Linux kernel does not support RDMA"),
				E::ENOMEM => panic!("Out of memory"),
				
				unexpected @ _ => panic!("ibv_get_device_list returned unexpected error number '{}' ('{}')", unexpected, errno),
			}
		}
		
		Self
		{
			list: list,
			size: size as usize,
			next: unsafe { *list },
			lifetime: PhantomData,
		}
	}
	
	#[inline(always)]
	pub fn is_empty(&self) -> bool
	{
		self.list.is_null()
	}
	
	#[inline(always)]
	pub fn len(&self) -> usize
	{
		self.size
	}
}
