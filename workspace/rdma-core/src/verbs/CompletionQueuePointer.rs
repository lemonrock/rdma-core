// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait CompletionQueuePointer
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq;
}

impl CompletionQueuePointer for *mut ibv_cq
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self
	}
}

impl CompletionQueuePointer for *mut ibv_cq_ex
{
	#[inline(always)]
	fn pointer(self) -> *mut ibv_cq
	{
		debug_assert!(!self.is_null(), "self is null");
		
		self.ibv_cq_ex_to_cq()
	}
}
