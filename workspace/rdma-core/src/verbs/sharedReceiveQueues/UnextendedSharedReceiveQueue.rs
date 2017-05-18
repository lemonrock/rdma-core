// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub struct UnextendedSharedReceiveQueue<'a>
{
	pub(crate) pointer: *mut ibv_srq,
	pub(crate) settings: SharedReceiveQueueSettings,
	pub(crate) protectionDomain: &'a ProtectionDomain<'a>
}

impl<'a> Drop for UnextendedSharedReceiveQueue<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(ibv_destroy_srq, self.pointer);
	}
}

impl<'a> SharedReceiveQueue for UnextendedSharedReceiveQueue<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_srq
	{
		self.pointer
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn settings(&mut self) -> &mut SharedReceiveQueueSettings
	{
		&mut self.settings
	}
	
	#[doc(hidden)]
	#[inline(always)]
	fn context(&self) -> &Context
	{
		self.protectionDomain.context
	}
	
	#[allow(trivial_casts)]
	#[doc(hidden)]
	#[inline(always)]
	fn isValidForProtectionDomain<'b>(&self, protectionDomain: &ProtectionDomain<'b>) -> bool
	{
		self.protectionDomain as *const _ == protectionDomain as *const _
	}
}
