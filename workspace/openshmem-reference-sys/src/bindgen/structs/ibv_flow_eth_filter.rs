// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct ibv_flow_eth_filter
{
	pub dst_mac: [u8; 6usize],
	pub src_mac: [u8; 6usize],
	pub ether_type: u16,
	pub vlan_tag: u16,
}

impl Clone for ibv_flow_eth_filter
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}
