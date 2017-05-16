// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct WithoutCompletionChannelUnextendedCompletionQueue<'a>
{
	pointer: *mut ibv_cq,
	context: &'a Context,
}

impl<'a> Drop for WithoutCompletionChannelUnextendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_cq, self.pointer);
	}
}

impl<'a> CompletionQueue for WithoutCompletionChannelUnextendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		self.pointer
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn isValidForContext(&self, context: &Context) -> bool
	{
		self.context as *const _ == context as *const _
	}
}

impl<'a> UnextendedCompletionQueue for WithoutCompletionChannelUnextendedCompletionQueue<'a>
{
}

impl<'a> WithoutCompletionChannelUnextendedCompletionQueue<'a>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut ibv_cq, context: &'a Context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			context: context,
		}
	}
}
