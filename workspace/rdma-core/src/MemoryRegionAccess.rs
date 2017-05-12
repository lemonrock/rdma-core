// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MemoryRegionAccess
{
	pub writeAccess: MemoryRegionWriteAccess,
	pub remoteRead: bool,
	pub isAMemoryWindow: bool,
	pub onDemand: bool
}

impl Default for MemoryRegionAccess
{
	#[inline(always)]
	fn default() -> Self
	{
		Self
		{
			writeAccess: Default::default(),
			remoteRead: true,
			isAMemoryWindow: false,
			onDemand: false,
		}
	}
}

impl MemoryRegionAccess
{
	#[inline(always)]
	pub fn as_c_int(&self) -> c_int
	{
		let mut access = self.writeAccess as i32;
		if likely(self.remoteRead)
		{
			const IBV_ACCESS_REMOTE_READ: i32 = 4;
			access = access + IBV_ACCESS_REMOTE_READ
		}
		if unlikely(self.isAMemoryWindow)
		{
			const IBV_ACCESS_MW_BIND: i32 = 16;
			access = access + IBV_ACCESS_MW_BIND
		}
		if unlikely(self.onDemand)
		{
			const IBV_ACCESS_ON_DEMAND: i32 = 64;
			access = access + IBV_ACCESS_ON_DEMAND
		}
		access
	}
}
