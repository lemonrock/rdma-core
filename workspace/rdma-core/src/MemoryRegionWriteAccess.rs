// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryRegionWriteAccess
{
	LocalWrites = 1, // IBV_ACCESS_LOCAL_WRITE
	RemoteWrites = 2, // IBV_ACCESS_REMOTE_ATOMIC
	LocalWritesAndRemoteWrites = 1 + 2, // IBV_ACCESS_LOCAL_WRITE + IBV_ACCESS_REMOTE_WRITE
	LocalWritesAndRemoteAtomicAccess = 1 + 8, // IBV_ACCESS_LOCAL_WRITE + IBV_ACCESS_REMOTE_ATOMIC
	LocalWritesAndRemoteWritesAndRemoteAtomicAccess = 1 + 2 + 8, // IBV_ACCESS_LOCAL_WRITE + IBV_ACCESS_REMOTE_WRITE + IBV_ACCESS_REMOTE_ATOMIC
}

impl Default for MemoryRegionWriteAccess
{
	#[inline(always)]
	fn default() -> Self
	{
		MemoryRegionWriteAccess::LocalWritesAndRemoteWrites
	}
}
