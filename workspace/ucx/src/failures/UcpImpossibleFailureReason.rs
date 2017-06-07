// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


/// Represents errors that just don't occur despite being defined in the API
#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UcpImpossibleFailureReason
{
	BufferTooSmall = ucs_status_t_UCS_ERR_BUFFER_TOO_SMALL,
	
	FailedToConnectToSomeOfTheRequestedEndPoints = ucs_status_t_UCS_ERR_SOME_CONNECTS_FAILED,
	
	OperationTimedOut = ucs_status_t_UCS_ERR_TIMED_OUT,
}
