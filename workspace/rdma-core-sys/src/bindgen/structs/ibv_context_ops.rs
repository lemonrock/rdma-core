// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct ibv_context_ops
{
	pub query_device: Option<unsafe extern "C" fn(context: *mut ibv_context, device_attr: *mut ibv_device_attr) -> c_int>,
	pub query_port: Option<unsafe extern "C" fn(context: *mut ibv_context, port_num: u8, port_attr: *mut ibv_port_attr) -> c_int>,
	pub alloc_pd: Option<unsafe extern "C" fn(context: *mut ibv_context) -> *mut ibv_pd>,
	pub dealloc_pd: Option<unsafe extern "C" fn(pd: *mut ibv_pd) -> c_int>,
	pub reg_mr: Option<unsafe extern "C" fn(pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> *mut ibv_mr>,
	pub rereg_mr: Option<unsafe extern "C" fn(mr: *mut ibv_mr, flags: c_int, pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> c_int>,
	pub dereg_mr: Option<unsafe extern "C" fn(mr: *mut ibv_mr) -> c_int>,
	pub alloc_mw: Option<unsafe extern "C" fn(pd: *mut ibv_pd, type_: ibv_mw_type) -> *mut ibv_mw>,
	pub bind_mw: Option<unsafe extern "C" fn(qp: *mut ibv_qp, mw: *mut ibv_mw, mw_bind: *mut ibv_mw_bind) -> c_int>,
	pub dealloc_mw: Option<unsafe extern "C" fn(mw: *mut ibv_mw) -> c_int>,
	pub create_cq: Option<unsafe extern "C" fn(context: *mut ibv_context, cqe: c_int, channel: *mut ibv_comp_channel, comp_vector: c_int) -> *mut ibv_cq>,
	pub poll_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq, num_entries: c_int, wc: *mut ibv_wc) -> c_int>,
	pub req_notify_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq, solicited_only: c_int) -> c_int>,
	pub cq_event: Option<unsafe extern "C" fn(cq: *mut ibv_cq)>,
	pub resize_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq, cqe: c_int) -> c_int>,
	pub destroy_cq: Option<unsafe extern "C" fn(cq: *mut ibv_cq) -> c_int>,
	pub create_srq: Option<unsafe extern "C" fn(pd: *mut ibv_pd, srq_init_attr: *mut ibv_srq_init_attr) -> *mut ibv_srq>,
	pub modify_srq: Option<unsafe extern "C" fn(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr, srq_attr_mask: c_int) -> c_int>,
	pub query_srq: Option<unsafe extern "C" fn(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr) -> c_int>,
	pub destroy_srq: Option<unsafe extern "C" fn(srq: *mut ibv_srq) -> c_int>,
	pub post_srq_recv: Option<unsafe extern "C" fn(srq: *mut ibv_srq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int>,
	pub create_qp: Option<unsafe extern "C" fn(pd: *mut ibv_pd, attr: *mut ibv_qp_init_attr) -> *mut ibv_qp>,
	pub query_qp: Option<unsafe extern "C" fn(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int, init_attr: *mut ibv_qp_init_attr) -> c_int>,
	pub modify_qp: Option<unsafe extern "C" fn(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int) -> c_int>,
	pub destroy_qp: Option<unsafe extern "C" fn(qp: *mut ibv_qp) -> c_int>,
	pub post_send: Option<unsafe extern "C" fn(qp: *mut ibv_qp, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr) -> c_int>,
	pub post_recv: Option<unsafe extern "C" fn(qp: *mut ibv_qp, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr) -> c_int>,
	pub create_ah: Option<unsafe extern "C" fn(pd: *mut ibv_pd, attr: *mut ibv_ah_attr) -> *mut ibv_ah>,
	pub destroy_ah: Option<unsafe extern "C" fn(ah: *mut ibv_ah) -> c_int>,
	pub attach_mcast: Option<unsafe extern "C" fn(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int>,
	pub detach_mcast: Option<unsafe extern "C" fn(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int>,
	pub async_event: Option<unsafe extern "C" fn(event: *mut ibv_async_event)>,
}

impl Default for ibv_context_ops
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
