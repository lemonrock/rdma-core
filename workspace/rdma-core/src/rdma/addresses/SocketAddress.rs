// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait SocketAddress: Sized
{
	#[inline(always)]
	fn as_sockaddr_ptr(&self) -> *const sockaddr;
	
	#[inline(always)]
	fn as_sockaddr_mut_ptr(&mut self) -> *mut sockaddr;
	
	#[inline(always)]
	fn as_sockaddr_storage_clone(&self) -> sockaddr_storage;
	
	#[inline(always)]
	fn family(&self) -> sa_family_t;
	
	#[inline(always)]
	fn toRdmaSocketAddress(self) -> RdmaSocketAddress;
	
	#[inline(always)]
	fn port(&self) -> Port;
}
