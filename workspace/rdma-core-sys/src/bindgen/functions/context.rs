// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ibv_alloc_pd(context: *mut ibv_context) -> *mut ibv_pd;
	pub fn ibv_close_device(context: *mut ibv_context) -> c_int;
	pub fn ibv_create_comp_channel(context: *mut ibv_context) -> *mut ibv_comp_channel;
	pub fn ibv_create_cq(context: *mut ibv_context, cqe: c_int, cq_context: *mut c_void, channel: *mut ibv_comp_channel, comp_vector: c_int) -> *mut ibv_cq;
	pub fn ibv_get_async_event(context: *mut ibv_context, event: *mut ibv_async_event) -> c_int;
	pub fn ibv_init_ah_from_wc(context: *mut ibv_context, port_num: u8, wc: *mut ibv_wc, grh: *mut ibv_grh, ah_attr: *mut ibv_ah_attr) -> c_int;
	pub fn ibv_query_device(context: *mut ibv_context, device_attr: *mut ibv_device_attr) -> c_int;
	pub fn ibv_query_gid(context: *mut ibv_context, port_num: u8, index: c_int, gid: *mut ibv_gid) -> c_int;
	pub fn ibv_query_pkey(context: *mut ibv_context, port_num: u8, index: c_int, pkey: *mut __be16) -> c_int;
	pub fn ibv_query_port(context: *mut ibv_context, port_num: u8, port_attr: *mut ibv_port_attr) -> c_int;
	pub fn ibv_resolve_eth_l2_from_gid(context: *mut ibv_context, attr: *mut ibv_ah_attr, eth_mac: *mut u8, vid: *mut u16) -> c_int;
}
