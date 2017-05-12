// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct MemoryRegion<'a>
{
	pointer: *mut ibv_mr,
	lifetime: PhantomData<&'a ProtectionDomain<'a>>
}

impl<'a> Drop for MemoryRegion<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_dereg_mr, self.pointer);
	}
}

impl<'a> MemoryRegion<'a>
{
	//	pub fn ibv_rereg_mr(mr: *mut ibv_mr, flags: c_int, pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> c_int
}

