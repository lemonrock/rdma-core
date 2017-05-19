// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(dead_code)]
pub struct WorkQueue<'a, C: CompletionQueue>
where C: 'a
{
	pub(crate) pointer: *mut ibv_wq,
	pub(crate) protectionDomain: &'a ProtectionDomain<'a>,
	pub(crate) completionQueue: &'a C,
	pub(crate) settings: SharedReceiveQueueSettings,
}

impl<'a, C: CompletionQueue> Drop for WorkQueue<'a, C>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(rust_ibv_destroy_wq, self.pointer)
	}
}

impl<'a, C: CompletionQueue> WorkQueue<'a, C>
{
	#[inline(always)]
	pub fn settings(&self) -> &SharedReceiveQueueSettings
	{
		&self.settings
	}
}
