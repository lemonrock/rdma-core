// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UnextendedQueuePair<'a>
{
	pointer: *mut ibv_qp,
	lifetime: &'a ProtectionDomain<'a>,
}

impl<'a> Drop for UnextendedQueuePair<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_qp, self.pointer);
	}
}

impl<'a> QueuePair<'a> for UnextendedQueuePair<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> * mut ibv_qp
	{
		self.pointer
	}
}
