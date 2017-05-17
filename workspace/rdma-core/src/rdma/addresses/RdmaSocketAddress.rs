// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub enum RdmaSocketAddress
{
	Infiniband(InfinibandSocketAddress),
	IpV4(IpV4SocketAddress),
	IpV6(IpV6SocketAddress),
}

impl SocketAddress for RdmaSocketAddress
{
	#[inline(always)]
	fn as_sockaddr_ptr(&self) -> *const sockaddr
	{
		match *self
		{
			RdmaSocketAddress::Infiniband(ref this) => this.as_sockaddr_ptr(),
			RdmaSocketAddress::IpV4(ref this) => this.as_sockaddr_ptr(),
			RdmaSocketAddress::IpV6(ref this) => this.as_sockaddr_ptr(),
		}
	}
	
	#[inline(always)]
	fn as_sockaddr_mut_ptr(&mut self) -> *mut sockaddr
	{
		match *self
		{
			RdmaSocketAddress::Infiniband(ref mut this) => this.as_sockaddr_mut_ptr(),
			RdmaSocketAddress::IpV4(ref mut this) => this.as_sockaddr_mut_ptr(),
			RdmaSocketAddress::IpV6(ref mut this) => this.as_sockaddr_mut_ptr(),
		}
	}
	
	#[inline(always)]
	fn as_sockaddr_storage_clone(&self) -> sockaddr_storage
	{
		match *self
		{
			RdmaSocketAddress::Infiniband(ref this) => this.as_sockaddr_storage_clone(),
			RdmaSocketAddress::IpV4(ref this) => this.as_sockaddr_storage_clone(),
			RdmaSocketAddress::IpV6(ref this) => this.as_sockaddr_storage_clone(),
		}
	}
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		match *self
		{
			RdmaSocketAddress::Infiniband(ref this) => this.family(),
			RdmaSocketAddress::IpV4(ref this) => this.family(),
			RdmaSocketAddress::IpV6(ref this) => this.family(),
		}
	}
	
	#[inline(always)]
	fn toRdmaSocketAddress(self) -> RdmaSocketAddress
	{
		self
	}
	
	#[inline(always)]
	fn port(&self) -> Port
	{
		match *self
		{
			RdmaSocketAddress::Infiniband(ref this) => this.port(),
			RdmaSocketAddress::IpV4(ref this) => this.port(),
			RdmaSocketAddress::IpV6(ref this) => this.port(),
		}
	}
}
