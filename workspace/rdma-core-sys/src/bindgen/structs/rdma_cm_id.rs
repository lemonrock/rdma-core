// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct rdma_cm_id
{
	pub verbs: *mut ibv_context,
	pub channel: *mut rdma_event_channel,
	pub context: *mut c_void,
	pub qp: *mut ibv_qp,
	pub route: rdma_route,
	pub ps: rdma_port_space,
	pub port_num: u8,
	pub event: *mut rdma_cm_event,
	pub send_cq_channel: *mut ibv_comp_channel,
	pub send_cq: *mut ibv_cq,
	pub recv_cq_channel: *mut ibv_comp_channel,
	pub recv_cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub pd: *mut ibv_pd,
	pub qp_type: ibv_qp_type,
}

impl Default for rdma_cm_id
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
