// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryDomain
{
	Wildcard,
	
	// Similar to, but not the same as, TransportLayerAliases for shared memory
	sysv,
	posix,
	xpmem,
}

impl MemoryDomain
{
	#[inline(always)]
	fn asMemoryAllocatorPriorityString(&self) -> &'static str
	{
		use self::MemoryDomain::*;
		
		match *self
		{
			Wildcard => "md:*",
			sysv => "md:sysv",
			posix => "md:posix",
			xpmem => "md:xpmem",
		}
	}
}
