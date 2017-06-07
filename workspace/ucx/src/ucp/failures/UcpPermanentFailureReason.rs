// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[repr(i8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UcpPermanentFailureReason
{
	OutOfMemory = ucs_status_t_UCS_ERR_NO_MEMORY,
	
	InvalidParameter = ucs_status_t_UCS_ERR_INVALID_PARAM,
	
	/// Failures with this reason occur early on because the address is just plain wrong
	/// *Except* for the UCS stats client, which returns this is gethostaddr() fails, ie if DNS fails
	InvalidRemoteAddressOrTcpAddressIsNotIpV6OrCanNotPackIntoRemoteAddressBuffer = ucs_status_t_UCS_ERR_INVALID_ADDR,
	
	UnimplementedFunctionality = ucs_status_t_UCS_ERR_NOT_IMPLEMENTED,
	
	/// Should occur quite early on; in essence, there are no suitable devices available for a given transport, eg we asked for InfiniBand and there are no InfiniBand cards / ports
	NoTransportDeviceExists = ucs_status_t_UCS_ERR_NO_DEVICE,
	
	/// Differs to UnimplementedFunctionality in that a particular function exists but a particular path of (reasonable from an user's perspective) logic through it is not supported
	UnsupportedSubSetOfFunctionality = ucs_status_t_UCS_ERR_UNSUPPORTED,
	
	/// Apart from configuration-time discovering that there are no devices (ucs_error), seems to indicate internal ucx programming failure
	ElementDoesNotExist = ucs_status_t_UCS_ERR_NO_ELEM,
	
	/// Seems to indicate internal ucx programming failure
	ElementAlreadyExists = ucs_status_t_UCS_ERR_ALREADY_EXISTS,
	
	/// Seems to indicate internal ucx programming failure; only used in stats code
	IndexOutOfRangeOrNameTooLong = ucs_status_t_UCS_ERR_OUT_OF_RANGE,
	
	/// It is believed that this should not leak up to the UCP API; seems to indicate internal ucx programming failure
	NoProgress = ucs_status_t_UCS_ERR_NO_PROGRESS,
}
