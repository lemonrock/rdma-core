// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Port(__be16);

impl Port
{
	#[inline(always)]
	pub fn as_network_endian(&self) -> __be16
	{
		self.0
	}
	
	#[inline(always)]
	pub fn as_host_endian(&self) -> u16
	{
		u16::from_be(self.0)
	}
	
	#[inline(always)]
	pub fn from_network_endian(port: __be16) -> Self
	{
		Port(port)
	}
	
	#[inline(always)]
	pub fn from_host_endian(port: u16) -> Self
	{
		Port(port.to_be())
	}
}
