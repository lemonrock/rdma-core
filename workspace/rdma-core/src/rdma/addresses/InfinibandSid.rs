// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InfinibandSid(__be64);

impl InfinibandSid
{
	#[inline(always)]
	pub fn as_network_endian(&self) -> __be64
	{
		self.0
	}
	
	#[inline(always)]
	pub fn as_host_endian(&self) -> u64
	{
		u64::from_be(self.0)
	}
	
	#[inline(always)]
	pub fn from_network_endian(sid: __be64) -> Self
	{
		InfinibandSid(sid)
	}
	
	#[inline(always)]
	pub fn from_host_endian(sid: u64) -> Self
	{
		InfinibandSid(sid.to_be())
	}
	
	#[cfg(target_endian = "big")]
	#[inline(always)]
	pub fn toPort(&self) -> Port
	{
		self.0 as u16
	}
	
	#[cfg(target_endian = "little")]
	#[inline(always)]
	pub fn toPort(&self) -> Port
	{
		Port::from_network_endian((self.0.to_le() as u16).to_be())
	}
}
