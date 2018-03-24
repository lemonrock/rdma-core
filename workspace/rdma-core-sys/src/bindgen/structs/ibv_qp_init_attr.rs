// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_qp_init_attr
{
	pub qp_context: *mut c_void,
	pub send_cq: *mut ibv_cq,
	pub recv_cq: *mut ibv_cq,
	pub srq: *mut ibv_srq,
	pub cap: ibv_qp_cap,
	pub qp_type: ibv_qp_type,
	pub sq_sig_all: c_int,
}

impl Default for ibv_qp_init_attr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_qp_init_attr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_qp_init_attr {{ qp_context: {:?}, send_cq: {:?}, recv_cq: {:?}, srq: {:?}, cap: {:?}, qp_type: {:?} }}", self.qp_context, self.send_cq, self.recv_cq, self.srq, self.cap, self.qp_type)
	}
}
