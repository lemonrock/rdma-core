// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct ExtendedCompletionQueue<'a>
{
	pointer: *mut ibv_cq_ex,
	lifetime: Option<&'a CompletionChannel<'a>>,
}

impl<'a> Drop for ExtendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		let pointer = self.pointer();
		panic_on_errno!(ibv_destroy_cq, pointer);
	}
}

impl<'a> CompletionQueue<'a> for ExtendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		unsafe { rust_ibv_cq_ex_to_cq(self.pointer) }
	}
}

impl<'a> ExtendedCompletionQueue<'a>
{
	#[inline(always)]
	fn new(pointer: *mut ibv_cq_ex, lifetime: Option<&'a CompletionChannel>) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
		}
	}
	
	// See https://www.mankier.com/3/ibv_create_cq_ex for other methods
}
