// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucm_event_type
{
	UCM_EVENT_MMAP = 1,
	UCM_EVENT_MUNMAP = 2,
	UCM_EVENT_MREMAP = 4,
	UCM_EVENT_SHMAT = 8,
	UCM_EVENT_SHMDT = 16,
	UCM_EVENT_SBRK = 32,
	UCM_EVENT_VM_MAPPED = 65536,
	UCM_EVENT_VM_UNMAPPED = 131072,
	UCM_EVENT_FLAG_NO_INSTALL = 16777216,
}
