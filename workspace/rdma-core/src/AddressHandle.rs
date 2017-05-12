// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct AddressHandle<'a>
{
	pointer: *mut ibv_ah,
	protectionDomain: &'a ProtectionDomain<'a>
}

impl<'a> Drop for AddressHandle<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_ah, self.pointer);
	}
}

//		pub fn ibv_create_ah_from_wc(pd: *mut ibv_pd, wc: *mut ibv_wc, grh: *mut ibv_grh, port_num: u8) -> *mut ibv_ah;
