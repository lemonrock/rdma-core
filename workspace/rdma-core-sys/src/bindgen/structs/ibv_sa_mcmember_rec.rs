// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_sa_mcmember_rec
{
	pub mgid: ibv_gid,
	pub port_gid: ibv_gid,
	pub qkey: u32,
	pub mlid: u16,
	pub mtu_selector: u8,
	pub mtu: u8,
	pub traffic_class: u8,
	pub pkey: u16,
	pub rate_selector: u8,
	pub rate: u8,
	pub packet_life_time_selector: u8,
	pub packet_life_time: u8,
	pub sl: u8,
	pub flow_label: u32,
	pub hop_limit: u8,
	pub scope: u8,
	pub join_state: u8,
	pub proxy_join: c_int,
}

impl Default for ibv_sa_mcmember_rec
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_sa_mcmember_rec
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_sa_mcmember_rec {{ mgid: {:?}, port_gid: {:?} }}", self.mgid, self.port_gid)
	}
}
