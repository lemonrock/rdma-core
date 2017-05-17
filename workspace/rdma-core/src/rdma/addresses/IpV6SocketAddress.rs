// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub struct IpV6SocketAddress
{
	inner: sockaddr_in6,
}

impl SocketAddress for IpV6SocketAddress
{
	#[allow(trivial_casts)]
	#[inline(always)]
	fn as_sockaddr_ptr(&self) -> *const sockaddr
	{
		&self.inner as *const _ as *const sockaddr
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn as_sockaddr_mut_ptr(&mut self) -> *mut sockaddr
	{
		&mut self.inner as *mut _ as *mut sockaddr
	}
	
	#[allow(trivial_casts)]
	#[inline(always)]
	fn as_sockaddr_storage_clone(&self) -> sockaddr_storage
	{
		let mut socketAddress = unsafe { zeroed() };
		unsafe { copy_nonoverlapping(&self.inner as *const _ as *const u8, &mut socketAddress as *mut _ as *mut u8, size_of::<sockaddr_in6>()); }
		socketAddress
	}
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		AF_INET6 as u8
	}
	
	#[inline(always)]
	fn toRdmaSocketAddress(self) -> RdmaSocketAddress
	{
		RdmaSocketAddress::IpV6(self)
	}
	
	#[inline(always)]
	fn port(&self) -> Port
	{
		Port(self.inner.sin6_port)
	}
}

impl SocketAddressCreator for IpV6SocketAddress
{
	#[inline(always)]
	fn localWithRandomPort() -> Self
	{
		let mut this = Self
		{
			inner: unsafe { zeroed() }
		};
		this.inner.sin6_family = AF_INET6 as sa_family_t;
		this
	}
}
