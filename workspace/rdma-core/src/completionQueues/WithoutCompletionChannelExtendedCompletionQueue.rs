// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct WithoutCompletionChannelExtendedCompletionQueue<'a>
{
	pub(crate) pointer: *mut ibv_cq_ex,
	lifetime: &'a Context,
	isCurrentlyBeingPolled: bool,
}

impl<'a> Drop for WithoutCompletionChannelExtendedCompletionQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		self.destroy();
	}
}

impl<'a> ExtendedCompletionQueue<'a> for WithoutCompletionChannelExtendedCompletionQueue<'a>
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

impl<'a> WithoutCompletionChannelExtendedCompletionQueue<'a>
{
	#[inline(always)]
	pub(crate) fn new(pointer: *mut ibv_cq_ex, lifetime: &'a Context) -> Self
	{
		debug_assert!(!pointer.is_null(), "pointer is null");
		
		Self
		{
			pointer: pointer,
			lifetime: lifetime,
			isCurrentlyBeingPolled: false,
		}
	}
}
