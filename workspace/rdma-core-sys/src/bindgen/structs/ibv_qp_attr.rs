// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_qp_attr
{
	pub qp_state: ibv_qp_state,
	pub cur_qp_state: ibv_qp_state,
	pub path_mtu: ibv_mtu,
	pub path_mig_state: ibv_mig_state,
	pub qkey: u32,
	pub rq_psn: u32,
	pub sq_psn: u32,
	pub dest_qp_num: u32,
	pub qp_access_flags: c_uint,
	pub cap: ibv_qp_cap,
	pub ah_attr: ibv_ah_attr,
	pub alt_ah_attr: ibv_ah_attr,
	pub pkey_index: u16,
	pub alt_pkey_index: u16,
	pub en_sqd_async_notify: u8,
	pub sq_draining: u8,
	pub max_rd_atomic: u8,
	pub max_dest_rd_atomic: u8,
	pub min_rnr_timer: u8,
	pub port_num: u8,
	pub timeout: u8,
	pub retry_cnt: u8,
	pub rnr_retry: u8,
	pub alt_port_num: u8,
	pub alt_timeout: u8,
	pub rate_limit: u32,
}

impl Default for ibv_qp_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_qp_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_qp_attr {{ qp_state: {:?}, cur_qp_state: {:?}, path_mtu: {:?}, path_mig_state: {:?}, cap: {:?}, ah_attr: {:?}, alt_ah_attr: {:?} }}", self.qp_state, self.cur_qp_state, self.path_mtu, self.path_mig_state, self.cap, self.ah_attr, self.alt_ah_attr)
	}
}
