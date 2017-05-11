// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ibv_ack_async_event(event: *mut ibv_async_event);
	pub fn ibv_ack_cq_events(cq: *mut ibv_cq, nevents: c_uint);
	pub fn ibv_alloc_pd(context: *mut ibv_context) -> *mut ibv_pd;
	pub fn ibv_attach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	pub fn ibv_close_device(context: *mut ibv_context) -> c_int;
	pub fn ibv_create_ah(pd: *mut ibv_pd, attr: *mut ibv_ah_attr) -> *mut ibv_ah;
	pub fn ibv_create_ah_from_wc(pd: *mut ibv_pd, wc: *mut ibv_wc, grh: *mut ibv_grh, port_num: u8) -> *mut ibv_ah;
	pub fn ibv_create_comp_channel(context: *mut ibv_context) -> *mut ibv_comp_channel;
	pub fn ibv_create_cq(context: *mut ibv_context, cqe: c_int, cq_context: *mut c_void, channel: *mut ibv_comp_channel, comp_vector: c_int) -> *mut ibv_cq;
	pub fn ibv_create_qp(pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> *mut ibv_qp;
	pub fn ibv_create_srq(pd: *mut ibv_pd, srq_init_attr: *mut ibv_srq_init_attr) -> *mut ibv_srq;
	pub fn ibv_dealloc_pd(pd: *mut ibv_pd) -> c_int;
	pub fn ibv_dereg_mr(mr: *mut ibv_mr) -> c_int;
	pub fn ibv_destroy_ah(ah: *mut ibv_ah) -> c_int;
	pub fn ibv_destroy_comp_channel(channel: *mut ibv_comp_channel) -> c_int;
	pub fn ibv_destroy_cq(cq: *mut ibv_cq) -> c_int;
	pub fn ibv_destroy_qp(qp: *mut ibv_qp) -> c_int;
	pub fn ibv_destroy_srq(srq: *mut ibv_srq) -> c_int;
	pub fn ibv_detach_mcast(qp: *mut ibv_qp, gid: *const ibv_gid, lid: u16) -> c_int;
	pub fn ibv_event_type_str(event: ibv_event_type) -> *const c_char;
	pub fn ibv_fork_init() -> c_int;
	pub fn ibv_free_device_list(list: *mut *mut ibv_device);
	pub fn ibv_get_async_event(context: *mut ibv_context, event: *mut ibv_async_event) -> c_int;
	pub fn ibv_get_cq_event(channel: *mut ibv_comp_channel, cq: *mut *mut ibv_cq, cq_context: *mut *mut c_void) -> c_int;
	pub fn ibv_get_device_guid(device: *mut ibv_device) -> u64;
	pub fn ibv_get_device_list(num_devices: *mut c_int) -> *mut *mut ibv_device;
	pub fn ibv_get_device_name(device: *mut ibv_device) -> *const c_char;
	pub fn ibv_init_ah_from_wc(context: *mut ibv_context, port_num: u8, wc: *mut ibv_wc, grh: *mut ibv_grh, ah_attr: *mut ibv_ah_attr) -> c_int;
	pub fn ibv_modify_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int) -> c_int;
	pub fn ibv_modify_srq(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr, srq_attr_mask: c_int) -> c_int;
	pub fn ibv_node_type_str(node_type: ibv_node_type) -> *const c_char;
	pub fn ibv_open_device(device: *mut ibv_device) -> *mut ibv_context;
	pub fn ibv_port_state_str(port_state: ibv_port_state) -> *const c_char;
	pub fn ibv_query_device(context: *mut ibv_context, device_attr: *mut ibv_device_attr) -> c_int;
	pub fn ibv_query_gid(context: *mut ibv_context, port_num: u8, index: c_int, gid: *mut ibv_gid) -> c_int;
	pub fn ibv_query_pkey(context: *mut ibv_context, port_num: u8, index: c_int, pkey: *mut u16) -> c_int;
	pub fn ibv_query_port(context: *mut ibv_context, port_num: u8, port_attr: *mut ibv_port_attr) -> c_int;
	pub fn ibv_query_qp(qp: *mut ibv_qp, attr: *mut ibv_qp_attr, attr_mask: c_int, init_attr: *mut ibv_qp_init_attr) -> c_int;
	pub fn ibv_query_srq(srq: *mut ibv_srq, srq_attr: *mut ibv_srq_attr) -> c_int;
	pub fn ibv_rate_to_mbps(rate: ibv_rate) -> c_int;
	pub fn ibv_rate_to_mult(rate: ibv_rate) -> c_int;
	pub fn ibv_reg_mr(pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> *mut ibv_mr;
	pub fn ibv_rereg_mr(mr: *mut ibv_mr, flags: c_int, pd: *mut ibv_pd, addr: *mut c_void, length: usize, access: c_int) -> c_int;
	pub fn ibv_resize_cq(cq: *mut ibv_cq, cqe: c_int) -> c_int;
	pub fn ibv_resolve_eth_l2_from_gid(context: *mut ibv_context, attr: *mut ibv_ah_attr, eth_mac: *mut u8, vid: *mut u16) -> c_int;
	pub fn ibv_wc_status_str(status: ibv_wc_status) -> *const c_char;
}
