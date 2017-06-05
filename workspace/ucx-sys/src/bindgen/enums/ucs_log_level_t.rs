// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucs_log_level_t
{
	UCS_LOG_LEVEL_FATAL = 0,
	UCS_LOG_LEVEL_ERROR = 1,
	UCS_LOG_LEVEL_WARN = 2,
	UCS_LOG_LEVEL_INFO = 3,
	UCS_LOG_LEVEL_DEBUG = 4,
	UCS_LOG_LEVEL_TRACE = 5,
	UCS_LOG_LEVEL_TRACE_REQ = 6,
	UCS_LOG_LEVEL_TRACE_DATA = 7,
	UCS_LOG_LEVEL_TRACE_ASYNC = 8,
	UCS_LOG_LEVEL_TRACE_FUNC = 9,
	UCS_LOG_LEVEL_TRACE_POLL = 10,
	UCS_LOG_LEVEL_LAST = 11,
}
