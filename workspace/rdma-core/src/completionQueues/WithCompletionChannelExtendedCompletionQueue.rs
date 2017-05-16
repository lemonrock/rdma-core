// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct WithCompletionChannelExtendedCompletionQueue<'a>
{
	pub(crate) pointer: *mut ibv_cq_ex,
	context: &'a Context,
	isCurrentlyBeingPolled: bool,
}

impl<'a> Drop for WithCompletionChannelExtendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.destroy();
	}
}

impl<'a> CompletionQueue for WithCompletionChannelExtendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq
	{
		unsafe { rust_ibv_cq_ex_to_cq(self.extendedPointer()) }
	}
	
	#[allow(trivial_casts)]
	#[doc(hidden)]
	#[inline(always)]
	fn isValidForContext(&self, context: &Context) -> bool
	{
		self.context as *const _ == context as *const _
	}
}

impl<'a> ExtendedCompletionQueue<'a> for WithCompletionChannelExtendedCompletionQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn extendedPointer(&self) -> *mut ibv_cq_ex
	{
		self.pointer
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn isCurrentlyBeingPolled(&self)-> bool
	{
		self.isCurrentlyBeingPolled
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn isNoLongerBeingPolled(&mut self)
	{
		self.isCurrentlyBeingPolled = false;
	}
}

impl<'a> WithCompletionChannelExtendedCompletionQueue<'a>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut ibv_cq_ex, context: &'a Context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			context: context,
			isCurrentlyBeingPolled: false,
		}
	}
}
