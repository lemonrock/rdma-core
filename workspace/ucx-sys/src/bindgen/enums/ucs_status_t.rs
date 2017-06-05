// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucs_status_t
{
	UCS_OK = 0,
	UCS_INPROGRESS = 1,
	UCS_ERR_NO_MESSAGE = -1,
	UCS_ERR_NO_RESOURCE = -2,
	UCS_ERR_IO_ERROR = -3,
	UCS_ERR_NO_MEMORY = -4,
	UCS_ERR_INVALID_PARAM = -5,
	UCS_ERR_UNREACHABLE = -6,
	UCS_ERR_INVALID_ADDR = -7,
	UCS_ERR_NOT_IMPLEMENTED = -8,
	UCS_ERR_MESSAGE_TRUNCATED = -9,
	UCS_ERR_NO_PROGRESS = -10,
	UCS_ERR_BUFFER_TOO_SMALL = -11,
	UCS_ERR_NO_ELEM = -12,
	UCS_ERR_SOME_CONNECTS_FAILED = -13,
	UCS_ERR_NO_DEVICE = -14,
	UCS_ERR_BUSY = -15,
	UCS_ERR_CANCELED = -16,
	UCS_ERR_SHMEM_SEGMENT = -17,
	UCS_ERR_ALREADY_EXISTS = -18,
	UCS_ERR_OUT_OF_RANGE = -19,
	UCS_ERR_TIMED_OUT = -20,
	UCS_ERR_EXCEEDS_LIMIT = -21,
	UCS_ERR_UNSUPPORTED = -22,
	UCS_ERR_FIRST_LINK_FAILURE = -40,
	UCS_ERR_LAST_LINK_FAILURE = -59,
	UCS_ERR_FIRST_ENDPOINT_FAILURE = -60,
	UCS_ERR_LAST_ENDPOINT_FAILURE = -79,
	UCS_ERR_ENDPOINT_TIMEOUT = -80,
	UCS_ERR_LAST = -100,
}
