// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(dead_code)]
pub struct MemoryWindow<'a>
{
	pointer: *mut ibv_mw,
	protectionDomain: &'a ProtectionDomain<'a>,
}

impl<'a> Drop for MemoryWindow<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(rust_ibv_dealloc_mw, self.pointer);
	}
}
