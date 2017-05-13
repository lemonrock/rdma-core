#include <infiniband/verbs.h>
#include "static-inline.h"

struct ibv_cq * rust_ibv_cq_ex_to_cq(struct ibv_cq_ex * cq)
{
	return ibv_cq_ex_to_cq(cq);
}

int rust_ibv_start_poll(struct ibv_cq_ex * cq, struct ibv_poll_cq_attr * attr)
{
	return ibv_start_poll(cq, attr);
}

int rust_ibv_next_poll(struct ibv_cq_ex * cq)
{
	return ibv_next_poll(cq);
}

void rust_ibv_end_poll(struct ibv_cq_ex * cq)
{
	ibv_end_poll(cq);
}

enum ibv_wc_opcode rust_ibv_wc_read_opcode(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_opcode(cq);
}

uint32_t rust_ibv_wc_read_vendor_err(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_vendor_err(cq);
}

uint32_t rust_ibv_wc_read_byte_len(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_byte_len(cq);
}

uint32_t rust_ibv_wc_read_imm_data(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_imm_data(cq);
}

uint32_t rust_ibv_wc_read_qp_num(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_qp_num(cq);
}

uint32_t rust_ibv_wc_read_src_qp(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_src_qp(cq);
}

int rust_ibv_wc_read_wc_flags(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_wc_flags(cq);
}

uint32_t rust_ibv_wc_read_slid(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_slid(cq);
}

uint8_t rust_ibv_wc_read_sl(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_sl(cq);
}

uint8_t rust_ibv_wc_read_dlid_path_bits(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_dlid_path_bits(cq);
}

uint64_t rust_ibv_wc_read_completion_ts(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_completion_ts(cq);
}

uint16_t rust_ibv_wc_read_cvlan(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_cvlan(cq);
}

uint32_t rust_ibv_wc_read_flow_tag(struct ibv_cq_ex * cq)
{
	return ibv_wc_read_flow_tag(cq);
}

int rust_ibv_post_wq_recv(struct ibv_wq * wq, struct ibv_recv_wr * recv_wr, struct ibv_recv_wr * * bad_recv_wr)
{
	return ibv_post_wq_recv(wq, recv_wr, bad_recv_wr);
}

struct verbs_context * rust_verbs_get_ctx(struct ibv_context * ctx)
{
	return verbs_get_ctx(ctx);
}

int rust____ibv_query_port(struct ibv_context * context, uint8_t port_num, struct ibv_port_attr * port_attr)
{
	return ___ibv_query_port(context, port_num, port_attr);
}

struct ibv_flow * rust_ibv_create_flow(struct ibv_qp * qp, struct ibv_flow_attr * flow)
{
	return ibv_create_flow(qp, flow);
}

int rust_ibv_destroy_flow(struct ibv_flow * flow_id)
{
	return ibv_destroy_flow(flow_id);
}

struct ibv_xrcd * rust_ibv_open_xrcd(struct ibv_context * context, struct ibv_xrcd_init_attr * xrcd_init_attr)
{
	return ibv_open_xrcd(context, xrcd_init_attr);
}

int rust_ibv_close_xrcd(struct ibv_xrcd * xrcd)
{
	return ibv_close_xrcd(xrcd);
}

struct ibv_mw * rust_ibv_alloc_mw(struct ibv_pd * pd, enum ibv_mw_type type)
{
	return ibv_alloc_mw(pd, type);
}

int rust_ibv_dealloc_mw(struct ibv_mw * mw)
{
	return ibv_dealloc_mw(mw);
}

uint32_t rust_ibv_inc_rkey(uint32_t rkey)
{
	return ibv_inc_rkey(rkey);
}

int rust_ibv_bind_mw(struct ibv_qp * qp, struct ibv_mw * mw, struct ibv_mw_bind * mw_bind)
{
	return ibv_bind_mw(qp, mw, mw_bind);
}

struct ibv_cq_ex * rust_ibv_create_cq_ex(struct ibv_context * context, struct ibv_cq_init_attr_ex * cq_attr)
{
	return ibv_create_cq_ex(context, cq_attr);
}

int rust_ibv_poll_cq(struct ibv_cq * cq, int num_entries, struct ibv_wc * wc)
{
	return ibv_poll_cq(cq, num_entries, wc);
}

int rust_ibv_req_notify_cq(struct ibv_cq * cq, int solicited_only)
{
	return ibv_req_notify_cq(cq, solicited_only);
}

struct ibv_srq * rust_ibv_create_srq_ex(struct ibv_context * context, struct ibv_srq_init_attr_ex * srq_init_attr_ex)
{
	return ibv_create_srq_ex(context, srq_init_attr_ex);
}

int rust_ibv_get_srq_num(struct ibv_srq * srq, uint32_t * srq_num)
{
	return ibv_get_srq_num(srq, srq_num);
}

int rust_ibv_post_srq_recv(struct ibv_srq * srq, struct ibv_recv_wr * recv_wr, struct ibv_recv_wr * * bad_recv_wr)
{
	return ibv_post_srq_recv(srq, recv_wr, bad_recv_wr);
}

struct ibv_qp * rust_ibv_create_qp_ex(struct ibv_context * context, struct ibv_qp_init_attr_ex * qp_init_attr_ex)
{
	return ibv_create_qp_ex(context, qp_init_attr_ex);
}

int rust_ibv_query_rt_values_ex(struct ibv_context * context, struct ibv_values_ex * values)
{
	return ibv_query_rt_values_ex(context, values);
}

int rust_ibv_query_device_ex(struct ibv_context * context, const struct ibv_query_device_ex_input * input, struct ibv_device_attr_ex * attr)
{
	return ibv_query_device_ex(context, input, attr);
}

struct ibv_qp * rust_ibv_open_qp(struct ibv_context * context, struct ibv_qp_open_attr * qp_open_attr)
{
	return ibv_open_qp(context, qp_open_attr);
}

struct ibv_wq * rust_ibv_create_wq(struct ibv_context * context, struct ibv_wq_init_attr * wq_init_attr)
{
	return ibv_create_wq(context, wq_init_attr);
}

int rust_ibv_modify_wq(struct ibv_wq * wq, struct ibv_wq_attr * wq_attr)
{
	return ibv_modify_wq(wq, wq_attr);
}

int rust_ibv_destroy_wq(struct ibv_wq * wq)
{
	return ibv_destroy_wq(wq);
}

struct ibv_rwq_ind_table * rust_ibv_create_rwq_ind_table(struct ibv_context * context, struct ibv_rwq_ind_table_init_attr * init_attr)
{
	return ibv_create_rwq_ind_table(context, init_attr);
}

int rust_ibv_destroy_rwq_ind_table(struct ibv_rwq_ind_table * rwq_ind_table)
{
	return ibv_destroy_rwq_ind_table(rwq_ind_table);
}

int rust_ibv_post_send(struct ibv_qp * qp, struct ibv_send_wr * wr, struct ibv_send_wr * * bad_wr)
{
	return ibv_post_send(qp, wr, bad_wr);
}

int rust_ibv_post_recv(struct ibv_qp * qp, struct ibv_recv_wr * wr, struct ibv_recv_wr * * bad_wr)
{
	return ibv_post_recv(qp, wr, bad_wr);
}

int rust_ibv_is_qpt_supported(uint32_t caps, enum ibv_qp_type qpt)
{
	return ibv_is_qpt_supported(caps, qpt);
}
