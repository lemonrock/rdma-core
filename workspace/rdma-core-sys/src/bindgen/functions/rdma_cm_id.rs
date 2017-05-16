// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn rdma_accept(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> c_int;
	pub fn rdma_bind_addr(id: *mut rdma_cm_id, addr: *mut sockaddr) -> c_int;
	pub fn rdma_connect(id: *mut rdma_cm_id, conn_param: *mut rdma_conn_param) -> c_int;
	pub fn rdma_create_ep(id: *mut *mut rdma_cm_id, res: *mut rdma_addrinfo, pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> c_int;
	pub fn rdma_create_qp(id: *mut rdma_cm_id, pd: *mut ibv_pd, qp_init_attr: *mut ibv_qp_init_attr) -> c_int;
	pub fn rdma_create_qp_ex(id: *mut rdma_cm_id, qp_init_attr: *mut ibv_qp_init_attr_ex) -> c_int;
	pub fn rdma_create_srq(id: *mut rdma_cm_id, pd: *mut ibv_pd, attr: *mut ibv_srq_init_attr) -> c_int;
	pub fn rdma_create_srq_ex(id: *mut rdma_cm_id, attr: *mut ibv_srq_init_attr_ex) -> c_int;
	pub fn rdma_destroy_ep(id: *mut rdma_cm_id);
	pub fn rdma_destroy_id(id: *mut rdma_cm_id) -> c_int;
	pub fn rdma_destroy_qp(id: *mut rdma_cm_id);
	pub fn rdma_destroy_srq(id: *mut rdma_cm_id);
	pub fn rdma_disconnect(id: *mut rdma_cm_id) -> c_int;
	pub fn rdma_get_dst_port(id: *mut rdma_cm_id) -> __be16;
	pub fn rdma_get_request(listen: *mut rdma_cm_id, id: *mut *mut rdma_cm_id) -> c_int;
	pub fn rdma_get_src_port(id: *mut rdma_cm_id) -> __be16;
	pub fn rdma_join_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr, context: *mut c_void) -> c_int;
	pub fn rdma_leave_multicast(id: *mut rdma_cm_id, addr: *mut sockaddr) -> c_int;
	pub fn rdma_listen(id: *mut rdma_cm_id, backlog: c_int) -> c_int;
	pub fn rdma_migrate_id(id: *mut rdma_cm_id, channel: *mut rdma_event_channel) -> c_int;
	pub fn rdma_notify(id: *mut rdma_cm_id, event: ibv_event_type) -> c_int;
	pub fn rdma_reject(id: *mut rdma_cm_id, private_data: *const c_void, private_data_len: u8) -> c_int;
	pub fn rdma_resolve_addr(id: *mut rdma_cm_id, src_addr: *mut sockaddr, dst_addr: *mut sockaddr, timeout_ms: c_int) -> c_int;
	pub fn rdma_resolve_route(id: *mut rdma_cm_id, timeout_ms: c_int) -> c_int;
	pub fn rdma_set_option(id: *mut rdma_cm_id, level: c_int, optname: c_int, optval: *mut c_void, optlen: usize) -> c_int;
	pub fn rust_rdma_get_recv_comp(id: *mut rdma_cm_id, wc: *mut ibv_wc) -> c_int;
	pub fn rust_rdma_get_send_comp(id: *mut rdma_cm_id, wc: *mut ibv_wc) -> c_int;
	pub fn rust_rdma_post_read(id: *mut rdma_cm_id, context: *mut c_void, addr: *mut c_void, length: usize, mr: *mut ibv_mr, flags: c_int, remote_addr: u64, rkey: u32) -> c_int;
	pub fn rust_rdma_post_readv(id: *mut rdma_cm_id, context: *mut c_void, sgl: *mut ibv_sge, nsge: c_int, flags: c_int, remote_addr: u64, rkey: u32) -> c_int;
	pub fn rust_rdma_post_recv(id: *mut rdma_cm_id, context: *mut c_void, addr: *mut c_void, length: usize, mr: *mut ibv_mr) -> c_int;
	pub fn rust_rdma_post_recvv(id: *mut rdma_cm_id, context: *mut c_void, sgl: *mut ibv_sge, nsge: c_int) -> c_int;
	pub fn rust_rdma_post_send(id: *mut rdma_cm_id, context: *mut c_void, addr: *mut c_void, length: usize, mr: *mut ibv_mr, flags: c_int) -> c_int;
	pub fn rust_rdma_post_sendv(id: *mut rdma_cm_id, context: *mut c_void, sgl: *mut ibv_sge, nsge: c_int, flags: c_int) -> c_int;
	pub fn rust_rdma_post_ud_send(id: *mut rdma_cm_id, context: *mut c_void, addr: *mut c_void, length: usize, mr: *mut ibv_mr, flags: c_int, ah: *mut ibv_ah, remote_qpn: u32) -> c_int;
	pub fn rust_rdma_post_write(id: *mut rdma_cm_id, context: *mut c_void, addr: *mut c_void, length: usize, mr: *mut ibv_mr, flags: c_int, remote_addr: u64, rkey: u32) -> c_int;
	pub fn rust_rdma_post_writev(id: *mut rdma_cm_id, context: *mut c_void, sgl: *mut ibv_sge, nsge: c_int, flags: c_int, remote_addr: u64, rkey: u32) -> c_int;
	pub fn rust_rdma_reg_msgs(id: *mut rdma_cm_id, addr: *mut c_void, length: usize) -> *mut ibv_mr;
	pub fn rust_rdma_reg_read(id: *mut rdma_cm_id, addr: *mut c_void, length: usize) -> *mut ibv_mr;
	pub fn rust_rdma_reg_write(id: *mut rdma_cm_id, addr: *mut c_void, length: usize) -> *mut ibv_mr;
}
