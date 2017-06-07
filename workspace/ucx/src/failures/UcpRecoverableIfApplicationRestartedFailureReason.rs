// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// NOTE: This means the application, NOT the application context
#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UcpRecoverableIfApplicationRestartedFailureReason
{
	/// epoll operations failed, or a read on a pipe failed; not recoverable as ucx does not give us enough information
	UnderlyingEPollOrLibcIoOperationFailed = ucs_status_t_UCS_ERR_IO_ERROR,
	
	/// Is used for open, truncate, read, write, close and delete; hides errors from calls like open() and shmat()
	/// Whilst in theory some of these errors are probably transient or recoverable, in practice, since we don't have any knowledge from ucx about what it was doing, we can't
	PosixOrSysVSharedMemoryError = ucs_status_t_UCS_ERR_SHMEM_SEGMENT,
	
	/// Aside from dealing with an InfiniBand verbs interface which has exceeded its usage of tags, should not occur
	IsEmptyOrIsFull	= ucs_status_t_UCS_ERR_EXCEEDS_LIMIT,
}
