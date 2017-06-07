// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryAllocatorPriority
{
	thp,
	md(MemoryDomain),
	heap,
	mmap,
	huge,
}

impl ToString for MemoryAllocatorPriority
{
	#[inline(always)]
	fn to_string(&self) -> String
	{
		use self::MemoryAllocatorPriority::*;
		
		match *self
		{
			thp => "thp",
			md(ref memoryDomain) => memoryDomain.asMemoryAllocatorPriorityString(),
			heap => "heap",
			mmap => "mmap",
			huge => "huge",
		}.to_owned()
	}
}

impl MemoryAllocatorPriority
{
	#[inline(always)]
	pub fn fromString(value: &str) -> Option<MemoryAllocatorPriority>
	{
		use self::MemoryAllocatorPriority::*;
		
		if value.is_empty()
		{
			None
		}
		else
		{
			match value
			{
				"thp" => Some(thp),
				"md:*" => Some(md(MemoryDomain::Wildcard)),
				"md:sysv" => Some(md(MemoryDomain::sysv)),
				"md:posix" => Some(md(MemoryDomain::posix)),
				"md:xpmem" => Some(md(MemoryDomain::xpmem)),
				"heap" => Some(heap),
				"mmap" => Some(mmap),
				"huge" => Some(huge),
				_ => None,
			}
		}
	}
}
