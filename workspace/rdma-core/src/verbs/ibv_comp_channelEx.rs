// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_comp_channelEx
{
	#[inline(always)]
	fn destroy(self);
	
	#[inline(always)]
	fn fileDescriptorForEpoll(self) -> RawFd;
	
	#[inline(always)]
	fn waitForCompletionEvent(self) -> (*mut ibv_cq, *mut c_void);
}

impl ibv_comp_channelEx for *mut ibv_comp_channel
{
	#[inline(always)]
	fn destroy(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		panic_on_errno!(ibv_destroy_comp_channel, self)
	}
	
	#[inline(always)]
	fn fileDescriptorForEpoll(self) -> RawFd
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { (*self).fd }
	}
	
	#[inline(always)]
	fn waitForCompletionEvent(self) -> (*mut ibv_cq, *mut c_void)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		let mut cq = null_mut();
		let mut context = null_mut();
		panic_on_error!(ibv_get_cq_event, self, &mut cq, &mut context);
		debug_assert!(!cq.is_null(), "cq should not be null");
		(cq, context)
	}
}
