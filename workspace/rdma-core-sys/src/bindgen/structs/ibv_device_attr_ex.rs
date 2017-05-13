// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_device_attr_ex
{
	pub orig_attr: ibv_device_attr,
	pub comp_mask: u32,
	pub odp_caps: ibv_odp_caps,
	pub completion_timestamp_mask: u64,
	pub hca_core_clock: u64,
	pub device_cap_flags_ex: u64,
	pub tso_caps: ibv_tso_caps,
	pub rss_caps: ibv_rss_caps,
	pub max_wq_type_rq: u32,
	pub packet_pacing_caps: ibv_packet_pacing_caps,
	pub raw_packet_caps: u32,
}

impl Default for ibv_device_attr_ex
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
