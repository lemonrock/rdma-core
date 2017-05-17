// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


include!("bindgen/structs/_bindgen_ty_3.rs");
include!("bindgen/structs/_bindgen_ty_4.rs");
include!("bindgen/structs/_ibv_device_ops.rs");
include!("bindgen/structs/ib_addr.rs");
include!("bindgen/structs/ibv_access_flags.rs");
include!("bindgen/structs/ibv_ah.rs");
include!("bindgen/structs/ibv_ah_attr.rs");
include!("bindgen/structs/ibv_async_event.rs");
include!("bindgen/structs/ibv_comp_channel.rs");
include!("bindgen/structs/ibv_context.rs");
include!("bindgen/structs/ibv_context_ops.rs");
include!("bindgen/structs/ibv_cq.rs");
include!("bindgen/structs/ibv_cq_ex.rs");
include!("bindgen/structs/ibv_cq_init_attr_ex.rs");
include!("bindgen/structs/ibv_cq_init_attr_mask.rs");
include!("bindgen/structs/ibv_create_cq_attr_flags.rs");
include!("bindgen/structs/ibv_create_cq_wc_flags.rs");
include!("bindgen/structs/ibv_device.rs");
include!("bindgen/structs/ibv_device_attr.rs");
include!("bindgen/structs/ibv_device_attr_ex.rs");
include!("bindgen/structs/ibv_flow.rs");
include!("bindgen/structs/ibv_flow_attr.rs");
include!("bindgen/structs/ibv_flow_eth_filter.rs");
include!("bindgen/structs/ibv_flow_ipv4_ext_filter.rs");
include!("bindgen/structs/ibv_flow_ipv4_filter.rs");
include!("bindgen/structs/ibv_flow_ipv6_filter.rs");
include!("bindgen/structs/ibv_flow_spec.rs");
include!("bindgen/structs/ibv_flow_spec__bindgen_ty_1__bindgen_ty_1.rs");
include!("bindgen/structs/ibv_flow_spec_action_tag.rs");
include!("bindgen/structs/ibv_flow_spec_eth.rs");
include!("bindgen/structs/ibv_flow_spec_ipv4.rs");
include!("bindgen/structs/ibv_flow_spec_ipv4_ext.rs");
include!("bindgen/structs/ibv_flow_spec_ipv6.rs");
include!("bindgen/structs/ibv_flow_spec_tcp_udp.rs");
include!("bindgen/structs/ibv_flow_spec_tunnel.rs");
include!("bindgen/structs/ibv_flow_tcp_udp_filter.rs");
include!("bindgen/structs/ibv_flow_tunnel_filter.rs");
include!("bindgen/structs/ibv_gid__bindgen_ty_1.rs");
include!("bindgen/structs/ibv_global_route.rs");
include!("bindgen/structs/ibv_grh.rs");
include!("bindgen/structs/ibv_mr.rs");
include!("bindgen/structs/ibv_mw.rs");
include!("bindgen/structs/ibv_mw_bind.rs");
include!("bindgen/structs/ibv_mw_bind_info.rs");
include!("bindgen/structs/ibv_odp_caps.rs");
include!("bindgen/structs/ibv_odp_caps__bindgen_ty_1.rs");
include!("bindgen/structs/ibv_packet_pacing_caps.rs");
include!("bindgen/structs/ibv_path_data.rs");
include!("bindgen/structs/ibv_path_record.rs");
include!("bindgen/structs/ibv_pd.rs");
include!("bindgen/structs/ibv_poll_cq_attr.rs");
include!("bindgen/structs/ibv_port_attr.rs");
include!("bindgen/structs/ibv_qp.rs");
include!("bindgen/structs/ibv_qp_attr.rs");
include!("bindgen/structs/ibv_qp_attr_mask.rs");
include!("bindgen/structs/ibv_qp_cap.rs");
include!("bindgen/structs/ibv_qp_init_attr.rs");
include!("bindgen/structs/ibv_qp_init_attr_ex.rs");
include!("bindgen/structs/ibv_qp_open_attr.rs");
include!("bindgen/structs/ibv_query_device_ex_input.rs");
include!("bindgen/structs/ibv_recv_wr.rs");
include!("bindgen/structs/ibv_rss_caps.rs");
include!("bindgen/structs/ibv_rwq_ind_table.rs");
include!("bindgen/structs/ibv_rwq_ind_table_init_attr.rs");
include!("bindgen/structs/ibv_rx_hash_conf.rs");
include!("bindgen/structs/ibv_sa_mcmember_rec.rs");
include!("bindgen/structs/ibv_sa_path_rec.rs");
include!("bindgen/structs/ibv_sa_service_rec.rs");
include!("bindgen/structs/ibv_send_flags.rs");
include!("bindgen/structs/ibv_send_wr.rs");
include!("bindgen/structs/ibv_send_wr__bindgen_ty_1__bindgen_ty_1.rs");
include!("bindgen/structs/ibv_send_wr__bindgen_ty_1__bindgen_ty_2.rs");
include!("bindgen/structs/ibv_send_wr__bindgen_ty_1__bindgen_ty_3.rs");
include!("bindgen/structs/ibv_send_wr__bindgen_ty_2__bindgen_ty_1.rs");
include!("bindgen/structs/ibv_send_wr__bindgen_ty_3__bindgen_ty_1.rs");
include!("bindgen/structs/ibv_send_wr__bindgen_ty_3__bindgen_ty_2.rs");
include!("bindgen/structs/ibv_sge.rs");
include!("bindgen/structs/ibv_srq.rs");
include!("bindgen/structs/ibv_srq_attr.rs");
include!("bindgen/structs/ibv_srq_init_attr.rs");
include!("bindgen/structs/ibv_srq_init_attr_ex.rs");
include!("bindgen/structs/ibv_srq_init_attr_mask.rs");
include!("bindgen/structs/ibv_tso_caps.rs");
include!("bindgen/structs/ibv_values_ex.rs");
include!("bindgen/structs/ibv_wc.rs");
include!("bindgen/structs/ibv_wc_flags.rs");
include!("bindgen/structs/ibv_wq.rs");
include!("bindgen/structs/ibv_wq_attr.rs");
include!("bindgen/structs/ibv_wq_init_attr.rs");
include!("bindgen/structs/ibv_xrcd.rs");
include!("bindgen/structs/ibv_xrcd_init_attr.rs");
include!("bindgen/structs/ibv_xrcd_init_attr_mask.rs");
include!("bindgen/structs/rdma_addr.rs");
include!("bindgen/structs/rdma_addrinfo.rs");
include!("bindgen/structs/rdma_cm_event.rs");
include!("bindgen/structs/rdma_cm_id.rs");
include!("bindgen/structs/rdma_conn_param.rs");
include!("bindgen/structs/rdma_event_channel.rs");
include!("bindgen/structs/rdma_ib_addr.rs");
include!("bindgen/structs/rdma_route.rs");
include!("bindgen/structs/rdma_ud_param.rs");
include!("bindgen/structs/sockaddr_ib.rs");
