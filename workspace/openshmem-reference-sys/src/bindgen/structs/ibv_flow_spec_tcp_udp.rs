// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_flow_spec_tcp_udp
{
	pub type_: ibv_flow_spec_type,
	pub size: u16,
	pub val: ibv_flow_tcp_udp_filter,
	pub mask: ibv_flow_tcp_udp_filter,
}

impl Clone for ibv_flow_spec_tcp_udp
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_flow_spec_tcp_udp
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
