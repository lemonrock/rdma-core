// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


use ::std::io;
use ::std::net::*;


pub struct DiscoveryLoop
{
	udpSocket: UdpSocket,
	configuration: UdpSocketMulticastConfiguration,
}

impl Drop for DiscoveryLoop
{
	#[inline(always)]
	fn drop(&mut self)
	{
		match self.configuration.closeUdpSocket(self.udpSocket)
		{
			Ok(()) => (),
			Err(_) => (),
		}
	}
}

impl DiscoveryLoop
{
	pub fn new(configuration: UdpSocketMulticastConfiguration) -> io::Result<Self>
	{
		Self
		{
			udpSocket: configuration.newUdpSocket(),
			configuration: configuration,
		}
	}
	
	pub fn run(&mut self)
	{
		let mut buf = [0; 512];
		
		let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
		
		
		
		self.udpSocket.send_to(&[0; 10], "127.0.0.1:4242").expect("couldn't send data");
	}
}

pub enum UdpSocketMulticastConfiguration
{
	// Local, multicast, multicast listen
	V4(SocketAddrV4, Ipv4Addr, Ipv4Addr),
	V6(SocketAddrV6, Ipv6Addr, u32),
}

impl UdpSocketMulticastConfiguration
{
	#[inline(always)]
	pub fn ipV4Any(local: SocketAddrV4, multicast: Ipv4Addr) -> Self
	{
		UdpSocketMulticastConfiguration::V4(local, multicast, Ipv4Addr::new(0, 0, 0, 0))
	}
	
	#[inline(always)]
	pub fn ipV6Any(local: Ipv6Addr, multicast: Ipv6Addr) -> Self
	{
		UdpSocketMulticastConfiguration::V6(local, multicast, 0)
	}
	
	#[inline(always)]
	pub fn newUdpSocket(&self) -> io::Result<UdpSocket>
	{
		use self::UdpSocketMulticastConfiguration::*;
		
		let	udpSocket = match *self
		{
			V4(ref localSocketAddress, ref multicastAddress, ref interface) =>
			{
				let	udpSocket = UdpSocket::bind(localSocketAddress)?;
				udpSocket.set_multicast_ttl_v4(1)?;
				udpSocket.set_multicast_loop_v4(false)?;
				udpSocket.join_multicast_v4(multicastAddress, interface)?;
				udpSocket
			},
			V6(ref localSocketAddress, ref multicastAddress, ref interface) =>
			{
				let	udpSocket = UdpSocket::bind(localSocketAddress)?;
				udpSocket.set_multicast_loop_v6(false)?;
				udpSocket.join_multicast_v6(multicastAddress, interface)?;
				udpSocket
			},
		};
		
		udpSocket.set_read_timeout(Some(5));
		udpSocket.set_write_timeout(Some(5));
		udpSocket.set_nonblocking(false);
		udpSocket.set_ttl(1);
		udpSocket.set_broadcast(true)?
	}
	
	#[inline(always)]
	pub fn closeUdpSocket(&self, udpSocket: UdpSocket) -> io::Result<()>
	{
		match *self
		{
			V4(ref localSocketAddress, ref multicastAddress, ref interface) =>
			{
				udpSocket.leave_multicast_v4(multicastAddress, interface);
			},
			V6(ref localSocketAddress, ref multicastAddress, ref interface) =>
			{
				udpSocket.leave_multicast_v6(multicastAddress, interface);
			},
		}
	}
}
