// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait QueuePair<'a>
{
	#[inline(always)]
	fn joinMultiCastGroup(&mut self, multiCastGroupIdentifier: MultiCastGroupIdentifier) -> bool;
	
	#[inline(always)]
	fn leaveMultiCastGroup(&mut self, multiCastGroupIdentifier: MultiCastGroupIdentifier) -> bool;
	
	/// See also <https://www.mankier.com/3/ibv_modify_qp>
	#[inline(always)]
	fn modify(&self, attributeMask: AttributeFlags::Flags, attributes: &mut ibv_qp_attr);
	
	/// See also <https://www.mankier.com/3/ibv_query_qp>
	#[inline(always)]
	fn attributes(&self) -> (ibv_qp_attr, ibv_qp_init_attr);
	
	/*
	
	pub fn rust_ibv_post_recv(qp: *mut ibv_qp, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr) -> c_int;
	pub fn rust_ibv_post_send(qp: *mut ibv_qp, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr) -> c_int;
	
	// creates a flow steering rule
	// https://www.mankier.com/3/ibv_create_flow
	pub fn rust_ibv_create_flow(qp: *mut ibv_qp, flow: *mut ibv_flow_attr) -> *mut ibv_flow;
	*/
}
