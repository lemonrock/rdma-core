// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait ibv_async_eventEx
{
	#[inline(always)]
	fn destroy(self);
}

impl ibv_async_eventEx for *mut ibv_async_event
{
	#[inline(always)]
	fn destroy(self)
	{
		debug_assert!(!self.is_null(), "self is null");
		
		unsafe { ibv_ack_async_event(self) }
	}
}

impl ibv_async_eventEx for ibv_async_event
{
	#[inline(always)]
	fn destroy(mut self)
	{
		unsafe { ibv_ack_async_event(&mut self) }
	}
}
