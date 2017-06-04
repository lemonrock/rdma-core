// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_error_t
{
	MXM_ERR_FIRST = -1,
	MXM_OK = 0,
	MXM_ERR_NO_MESSAGE = 1,
	MXM_ERR_WOULD_BLOCK = 2,
	MXM_ERR_IO_ERROR = 3,
	MXM_ERR_NO_MEMORY = 4,
	MXM_ERR_INVALID_PARAM = 5,
	MXM_ERR_UNREACHABLE = 6,
	MXM_ERR_INVALID_ADDR = 7,
	MXM_ERR_NOT_IMPLEMENTED = 8,
	MXM_ERR_MESSAGE_TRUNCATED = 9,
	MXM_ERR_NO_PROGRESS = 10,
	MXM_ERR_BUFFER_TOO_SMALL = 11,
	MXM_ERR_NO_ELEM = 12,
	MXM_ERR_SOME_CONNECTS_FAILED = 13,
	MXM_ERR_NO_DEVICE = 14,
	MXM_ERR_BUSY = 15,
	MXM_ERR_CANCELED = 16,
	MXM_ERR_SHMEM_SEGMENT = 17,
	MXM_ERR_ALREADY_EXISTS = 18,
	MXM_ERR_OUT_OF_RANGE = 19,
	MXM_ERR_TIMED_OUT = 20,
	MXM_ERR_EXCEEDS_LIMIT = 21,
	MXM_ERR_UNSUPPORTED = 22,
	MXM_ERR_LAST = 23,
}
