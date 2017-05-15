// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


pub trait QueuePair<'a>
{
	#[doc(hidden)]
	#[inline(always)]
	fn pointer(&self) -> *mut ibv_qp;
	
	/*
	
	pub fn ibv_modify_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int) -> c_int;
	pub fn ibv_query_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int, init_attr: *mut ibv_qp_init_attr) -> c_int;
	
	pub fn rust_ibv_post_recv(qp: *mut ibv_qp, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr) -> c_int;
	pub fn rust_ibv_post_send(qp: *mut ibv_qp, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr) -> c_int;
	
	// Must be detached before being drop'd
	pub fn ibv_attach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	pub fn ibv_detach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	
	// creates a flow steering rule
	// https://www.mankier.com/3/ibv_create_flow
	pub fn rust_ibv_create_flow(qp: *mut ibv_qp, flow: *mut ibv_flow_attr) -> *mut ibv_flow;
	*/
}

#[derive(Copy, Clone)]
pub struct MultiCastGroupIdentifier
{
	multicastGroupId: ibv_gid,
	localIdentifier: LocalIdentifier,
}

impl MultiCastGroupIdentifier
{
	#[inline(always)]
	pub fn new(subnetPrefix: __be64, interfaceId: __be64, localIdentifier: LocalIdentifier) -> Self
	{
		Self
		{
			multicastGroupId: ibv_gid
			{
				global: ibv_gid__bindgen_ty_1
				{
					subnet_prefix: subnetPrefix,
					interface_id: interfaceId,
				}
			},
			localIdentifier: localIdentifier,
		}
	}
}
