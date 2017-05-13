// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct UnextendedCompletionQueue<'a>
{
	pointer: *mut ibv_cq,
	lifetime: Option<&'a CompletionChannel<'a>>,
}

impl<'a> Drop for UnextendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_cq, self.pointer);
	}
}

impl<'a> CompletionQueue<'a> for UnextendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		self.pointer
	}
}

impl<'a> UnextendedCompletionQueue<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_cq, lifetime: Option<&'a CompletionChannel>) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
		}
	}
}
