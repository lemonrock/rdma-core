// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub trait CompletionQueue: Drop
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_cq;
	
	#[doc(hidden)]
	#[inline(always)]
	fn isValidForContext(&self, context: &Context) -> bool;
	
	#[inline(always)]
	fn resize(&self, atLeastThisNumberOfCompletionQueueEvents: u31)
	{
		panic_on_error!(ibv_resize_cq, self.pointer(), atLeastThisNumberOfCompletionQueueEvents as i32);
	}
}
