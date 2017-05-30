// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct ibv_port_attr
{
	pub state: ibv_port_state,
	pub max_mtu: ibv_mtu,
	pub active_mtu: ibv_mtu,
	pub gid_tbl_len: c_int,
	pub port_cap_flags: u32,
	pub max_msg_sz: u32,
	pub bad_pkey_cntr: u32,
	pub qkey_viol_cntr: u32,
	pub pkey_tbl_len: u16,
	pub lid: u16,
	pub sm_lid: u16,
	pub lmc: u8,
	pub max_vl_num: u8,
	pub sm_sl: u8,
	pub subnet_timeout: u8,
	pub init_type_reply: u8,
	pub active_width: u8,
	pub active_speed: u8,
	pub phys_state: u8,
	pub link_layer: u8,
	pub reserved: u8,
}

impl Clone for ibv_port_attr
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for ibv_port_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
