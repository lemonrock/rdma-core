// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn rust_ibv_alloc_mw(pd: *mut ibv_pd, type_: ibv_mw_type) -> *mut ibv_mw;
	pub fn rust_ibv_alloc_parent_domain(context: *mut ibv_context, attr: *mut ibv_parent_domain_init_attr) -> *mut ibv_pd;
	pub fn rust_ibv_alloc_td(context: *mut ibv_context, init_attr: *mut ibv_td_init_attr) -> *mut ibv_td;
	pub fn rust_ibv_bind_mw(qp: *mut ibv_qp, mw: *mut ibv_mw, mw_bind: *mut ibv_mw_bind) -> c_int;
	pub fn rust_ibv_close_xrcd(xrcd: *mut ibv_xrcd) -> c_int;
	pub fn rust_ibv_cq_ex_to_cq(cq: *mut ibv_cq_ex) -> *mut ibv_cq;
	pub fn rust_ibv_create_cq_ex(context: *mut ibv_context, cq_attr: *mut ibv_cq_init_attr_ex) -> *mut ibv_cq_ex;
	pub fn rust_ibv_create_flow(qp: *mut ibv_qp, flow: *mut ibv_flow_attr) -> *mut ibv_flow;
	pub fn rust_ibv_create_qp_ex(context: *mut ibv_context, qp_init_attr_ex: *mut ibv_qp_init_attr_ex) -> *mut ibv_qp;
	pub fn rust_ibv_create_rwq_ind_table(context: *mut ibv_context, init_attr: *mut ibv_rwq_ind_table_init_attr) -> *mut ibv_rwq_ind_table;
	pub fn rust_ibv_create_srq_ex(context: *mut ibv_context, srq_init_attr_ex: *mut ibv_srq_init_attr_ex) -> *mut ibv_srq;
	pub fn rust_ibv_create_wq(context: *mut ibv_context, wq_init_attr: *mut ibv_wq_init_attr) -> *mut ibv_wq;
	pub fn rust_ibv_dealloc_mw(mw: *mut ibv_mw) -> c_int;
	pub fn rust_ibv_dealloc_td(td: *mut ibv_td) -> c_int;
	pub fn rust_ibv_destroy_flow(flow_id: *mut ibv_flow) -> c_int;
	pub fn rust_ibv_destroy_rwq_ind_table(rwq_ind_table: *mut ibv_rwq_ind_table) -> c_int;
	pub fn rust_ibv_destroy_wq(wq: *mut ibv_wq) -> c_int;
	pub fn rust_ibv_end_poll(cq: *mut ibv_cq_ex);
	pub fn rust_ibv_get_srq_num(srq: *mut ibv_srq, srq_num: *mut u32) -> c_int;
	pub fn rust_ibv_inc_rkey(rkey: u32) -> u32;
	pub fn rust_ibv_is_qpt_supported(caps: u32, qpt: ibv_qp_type) -> c_int;
	pub fn rust_ibv_modify_cq(cq: *mut ibv_cq, attr: *mut ibv_modify_cq_attr) -> c_int;
	pub fn rust_ibv_modify_qp_rate_limit(qp: *mut ibv_qp, attr: *mut ibv_qp_rate_limit_attr) -> c_int;
	pub fn rust_ibv_modify_wq(wq: *mut ibv_wq, wq_attr: *mut ibv_wq_attr) -> c_int;
	pub fn rust_ibv_next_poll(cq: *mut ibv_cq_ex) -> c_int;
	pub fn rust_ibv_open_qp(context: *mut ibv_context, qp_open_attr: *mut ibv_qp_open_attr) -> *mut ibv_qp;
	pub fn rust_ibv_open_xrcd(context: *mut ibv_context, xrcd_init_attr: *mut ibv_xrcd_init_attr) -> *mut ibv_xrcd;
	pub fn rust_ibv_poll_cq(cq: *mut ibv_cq, num_entries: c_int, wc: *mut ibv_wc) -> c_int;
	pub fn rust_ibv_post_recv(qp: *mut ibv_qp, wr: *mut ibv_recv_wr, bad_wr: *mut *mut ibv_recv_wr) -> c_int;
	pub fn rust_ibv_post_send(qp: *mut ibv_qp, wr: *mut ibv_send_wr, bad_wr: *mut *mut ibv_send_wr) -> c_int;
	pub fn rust_ibv_post_srq_ops(srq: *mut ibv_srq, op: *mut ibv_ops_wr, bad_op: *mut *mut ibv_ops_wr) -> c_int;
	pub fn rust_ibv_post_srq_recv(srq: *mut ibv_srq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int;
	pub fn rust_ibv_post_wq_recv(wq: *mut ibv_wq, recv_wr: *mut ibv_recv_wr, bad_recv_wr: *mut *mut ibv_recv_wr) -> c_int;
	pub fn rust_ibv_query_device_ex(context: *mut ibv_context, input: *const ibv_query_device_ex_input, attr: *mut ibv_device_attr_ex) -> c_int;
	pub fn rust_ibv_query_rt_values_ex(context: *mut ibv_context, values: *mut ibv_values_ex) -> c_int;
	pub fn rust_ibv_req_notify_cq(cq: *mut ibv_cq, solicited_only: c_int) -> c_int;
	pub fn rust_ibv_start_poll(cq: *mut ibv_cq_ex, attr: *mut ibv_poll_cq_attr) -> c_int;
	pub fn rust_ibv_wc_read_byte_len(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_completion_ts(cq: *mut ibv_cq_ex) -> u64;
	pub fn rust_ibv_wc_read_completion_wallclock_ns(cq: *mut ibv_cq_ex) -> u64;
	pub fn rust_ibv_wc_read_cvlan(cq: *mut ibv_cq_ex) -> u16;
	pub fn rust_ibv_wc_read_dlid_path_bits(cq: *mut ibv_cq_ex) -> u8;
	pub fn rust_ibv_wc_read_flow_tag(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_imm_data(cq: *mut ibv_cq_ex) -> __be32;
	pub fn rust_ibv_wc_read_invalidated_rkey(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_opcode(cq: *mut ibv_cq_ex) -> ibv_wc_opcode;
	pub fn rust_ibv_wc_read_qp_num(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_sl(cq: *mut ibv_cq_ex) -> u8;
	pub fn rust_ibv_wc_read_slid(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_src_qp(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_tm_info(cq: *mut ibv_cq_ex, tm_info: *mut ibv_wc_tm_info);
	pub fn rust_ibv_wc_read_vendor_err(cq: *mut ibv_cq_ex) -> u32;
	pub fn rust_ibv_wc_read_wc_flags(cq: *mut ibv_cq_ex) -> c_uint;
}
