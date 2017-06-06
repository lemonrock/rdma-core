// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
pub struct ApplicationContextAttributes(ucp_context_attr);

impl ApplicationContextAttributes
{
	#[inline(always)]
	fn threadMode(&self) -> ucs_thread_mode_t
	{
		self.0.thread_mode
	}
	
	#[inline(always)]
	pub fn workerThreadMode(&self) -> WorkerThreadMode
	{
		use ucs_thread_mode_t::*;
		use WorkerThreadMode::*;
		
		match self.threadMode()
		{
			UCS_THREAD_MODE_SINGLE => OnlyEverUsedFromThisThread,
			UCS_THREAD_MODE_SERIALIZED => SerializedOneThreadAtATime,
			UCS_THREAD_MODE_MULTI => UsedSimultaneouslyAcrossMoreThanOneThread,
			UCS_THREAD_MODE_LAST => panic!("Invalid thread mode 'UCS_THREAD_MODE_LAST'"),
		}
	}
	
	#[inline(always)]
	pub fn requestSize(&self) -> usize
	{
		self.0.request_size
	}
}

impl PartialEq for ApplicationContextAttributes
{
	#[inline(always)]
	fn eq(&self, other: &ApplicationContextAttributes) -> bool
	{
		self.threadMode() == other.threadMode() && self.requestSize() == other.requestSize()
	}
}

impl Eq for ApplicationContextAttributes
{
}

impl PartialOrd for ApplicationContextAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &ApplicationContextAttributes) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for ApplicationContextAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &ApplicationContextAttributes) -> Ordering
	{
		self.workerThreadMode().cmp(&other.workerThreadMode()).then(self.requestSize().cmp(&other.requestSize()))
	}
}

impl Hash for ApplicationContextAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.threadMode().hash(state);
		self.requestSize().hash(state);
	}
}
