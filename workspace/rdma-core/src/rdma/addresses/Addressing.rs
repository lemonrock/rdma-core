// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Addressing
{
	UnreliableUnconnectedUdpOverIpV4OverEthernet,
	UnreliableUnconnectedUdpOverIpV6OverEthernet,
	UnreliableUnconnectedIpV4OverInfiniband,
	ReliableConnectedTcpOverIpV4OverEthernet,
	ReliableConnectedTcpOverIpVOver6Ethernet,
	ReliableConnectedOverInfiniband,
}

impl Addressing
{
	#[inline(always)]
	pub(crate) fn createForLocal(&self) -> (RdmaSocketAddress, rdma_port_space)
	{
		use self::Addressing::*;
		use ::rdma_core_sys::rdma_port_space::*;
		
		match *self
		{
			UnreliableUnconnectedUdpOverIpV4OverEthernet => (IpV4SocketAddress::newRdmaSocketAddress(), RDMA_PS_UDP),
			UnreliableUnconnectedUdpOverIpV6OverEthernet => (IpV6SocketAddress::newRdmaSocketAddress(), RDMA_PS_UDP),
			UnreliableUnconnectedIpV4OverInfiniband => (IpV4SocketAddress::newRdmaSocketAddress(), RDMA_PS_IPOIB),
			ReliableConnectedTcpOverIpV4OverEthernet => (IpV4SocketAddress::newRdmaSocketAddress(), RDMA_PS_TCP),
			ReliableConnectedTcpOverIpVOver6Ethernet => (IpV6SocketAddress::newRdmaSocketAddress(), RDMA_PS_TCP),
			ReliableConnectedOverInfiniband => (InfinibandSocketAddress::newRdmaSocketAddress(), RDMA_PS_IB),
		}
	}
}
