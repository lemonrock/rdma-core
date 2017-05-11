// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProtectionDomain<'a>
{
	pointer: *mut ibv_pd,
	lifetime: PhantomData<&'a Context>,
}

impl<'a> Drop for ProtectionDomain<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_dealloc_pd, self.pointer);
	}
}

impl<'a> ProtectionDomain<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_pd) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: PhantomData,
		}
	}
}
