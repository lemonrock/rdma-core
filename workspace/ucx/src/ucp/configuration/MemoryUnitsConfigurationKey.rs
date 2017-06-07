// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MemoryUnitsConfigurationKey
{
	/// Default is `0`
	ThresholdForSwitchingFromShortToBufferCopyProtocol,
	
	/// Default is `auto`
	ThresholdForSwitchingFromEagerToRendezvousProtocol,
	
	/// Default is `inf`
	MessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative,
	
	/// Default is `auto`
	ThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol,
	
	/// Default is `5800mb`
	EstimationOfBufferCopyBandwidth,
	
	/// Default is `1024`
	/// Buffer smaller than this value will not be posted to the transport
	ThresholdForUsingTagMatchingOffloadCapabilities,
}

impl ConfigurationKey for MemoryUnitsConfigurationKey
{
	#[inline(always)]
	fn key(&self) -> *const c_char
	{
		const_cstr!
		{
			BCOPY_THRESH = "BCOPY_THRESH";
			RNDV_THRESH = "RNDV_THRESH";
			RNDV_THRESH_FALLBACK = "RNDV_THRESH_FALLBACK";
			ZCOPY_THRESH = "ZCOPY_THRESH";
			BCOPY_BW = "BCOPY_BW";
			TM_THRESH = "TM_THRESH";
		}
		
		use self::MemoryUnitsConfigurationKey::*;
		
		let constant = match *self
		{
			ThresholdForSwitchingFromShortToBufferCopyProtocol => BCOPY_THRESH,
			ThresholdForSwitchingFromEagerToRendezvousProtocol => RNDV_THRESH,
			MessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative => RNDV_THRESH_FALLBACK,
			ThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol => ZCOPY_THRESH,
			EstimationOfBufferCopyBandwidth => BCOPY_BW,
			ThresholdForUsingTagMatchingOffloadCapabilities => TM_THRESH,
		};
		constant.as_ptr()
	}
	
	#[inline(always)]
	fn defaultValue(&self) -> CString
	{
		use self::MemoryUnitsConfigurationKey::*;
		
		let value = match *self
		{
			ThresholdForSwitchingFromShortToBufferCopyProtocol => "0",
			ThresholdForSwitchingFromEagerToRendezvousProtocol => "auto",
			MessageSizeThresholdToStartUsingTheRendezvousProtocolInCaseTheCalculatedThresholdIsZeroOrNegative => "inf",
			ThresholdForSwitchingFromBufferCopyProtocolToZeroCopyProtocol => "auto",
			EstimationOfBufferCopyBandwidth => "5800mb",
			ThresholdForUsingTagMatchingOffloadCapabilities => "1024",
		};
		CString::new(value).unwrap()
	}
}
