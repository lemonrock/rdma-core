// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone)]
pub struct WorkerAttributes(ucp_worker_attr_t);

impl WorkerAttributes
{
	#[inline(always)]
	fn threadMode(&self) -> ucs_thread_mode_t
	{
		self.0.thread_mode
	}
	
	/// NOTE: SerializedOneThreadAtATime is NEVER returned by ucx currently
	#[inline(always)]
	pub fn workerThreadMode(&self) -> WorkerThreadMode
	{
		use ucs_thread_mode_t::*;
		use self::WorkerThreadMode::*;
		
		match self.threadMode()
		{
			UCS_THREAD_MODE_SINGLE => OnlyEverUsedFromThisThread,
			UCS_THREAD_MODE_SERIALIZED => SerializedOneThreadAtATime,
			UCS_THREAD_MODE_MULTI => UsedSimultaneouslyAcrossMoreThanOneThread,
			UCS_THREAD_MODE_LAST => panic!("Invalid thread mode 'UCS_THREAD_MODE_LAST'"),
		}
	}
}

impl PartialEq for WorkerAttributes
{
	#[inline(always)]
	fn eq(&self, other: &WorkerAttributes) -> bool
	{
		self.threadMode() == other.threadMode()
	}
}

impl Eq for WorkerAttributes
{
}

impl PartialOrd for WorkerAttributes
{
	#[inline(always)]
	fn partial_cmp(&self, other: &WorkerAttributes) -> Option<Ordering>
	{
		Some(self.cmp(other))
	}
}

impl Ord for WorkerAttributes
{
	#[inline(always)]
	fn cmp(&self, other: &WorkerAttributes) -> Ordering
	{
		self.workerThreadMode().cmp(&other.workerThreadMode())
	}
}

impl Hash for WorkerAttributes
{
	#[inline(always)]
	fn hash<H: Hasher>(&self, state: &mut H)
	{
		self.threadMode().hash(state);
	}
}
