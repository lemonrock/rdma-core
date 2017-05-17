// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Copy, Clone)]
pub struct InfinibandSocketAddress
{
	inner: sockaddr_ib,
}

impl SocketAddress for InfinibandSocketAddress
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
		unsafe { copy_nonoverlapping(&self.inner as *const _ as *const u8, &mut socketAddress as *mut _ as *mut u8, size_of::<sockaddr_ib>()); }
		socketAddress
	}
	
	#[inline(always)]
	fn family(&self) -> sa_family_t
	{
		AF_IB
	}
	
	#[inline(always)]
	fn toRdmaSocketAddress(self) -> RdmaSocketAddress
	{
		RdmaSocketAddress::Infiniband(self)
	}
	
	#[inline(always)]
	fn port(&self) -> Port
	{
		self.sid().toPort()
	}
}

/// An example of populating sockaddr_ib is at the bottom of here: <https://www.spinics.net/lists/linux-rdma/msg22377.html>
impl SocketAddressCreator for InfinibandSocketAddress
{
	#[inline(always)]
	fn localWithRandomPort() -> Self
	{
		// Missing from rdma-core's librdmacm/rdma_cma.h (installed as rdma/rdma_cma.h), but present in Linux sources near-equivalent include/rdma/rdma_cm.h header
		const RDMA_IB_IP_PS_IB: c_uint = RDMA_IB_PS_IB;
		
		let serverPort: u16 = 0;
		
		Self
		{
			inner: sockaddr_ib
			{
				sib_family: AF_IB as u16,
				sib_pkey: 0xFFFFu16.to_be(),
				sib_flowinfo: 0u32.to_be(),
				sib_addr: ib_addr
				{
					ib_u: ib_addr__bindgen_ty_1::default(),
				},
				sib_sid: (RDMA_IB_IP_PS_IB as u64 | serverPort as u64).to_be(),
				sib_sid_mask: 0xFFFFFFFFFFFFFFFFu64.to_be(),
				sib_scope_id: 0u64.to_be(),
			}
		}
	}
}

impl InfinibandSocketAddress
{
	#[inline(always)]
	fn sid(&self) -> InfinibandSid
	{
		InfinibandSid::from_network_endian(self.inner.sib_sid)
	}
}
