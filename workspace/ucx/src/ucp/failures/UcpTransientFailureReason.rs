// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UcpTransientFailureReason
{
	/// Only seems to be relevant to receiving
	/// Does not seem to ever escape stats internal code
	NoPendingMessage = ucs_status_t_UCS_ERR_NO_MESSAGE,
	
	/// Seems to be caused by flush(); try-again
	NoResource = ucs_status_t_UCS_ERR_NO_RESOURCE,
	
	/// Usually called because we can not connect to the remote memory access key (rkey)
	/// Tear down RemoteMemoryAccessKey and probably EndPoint
	DestinationAddressIsUnreachable = ucs_status_t_UCS_ERR_UNREACHABLE,
	
	/// Tear down RemoteMemoryAccessKey and probably EndPoint
	EndPointTimeOut = ucs_status_t_UCS_ERR_ENDPOINT_TIMEOUT,
}
