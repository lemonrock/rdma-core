// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub const ibv_device_cap_flags_IBV_DEVICE_AUTO_PATH_MIG: ibv_device_cap_flags = ibv_device_cap_flags(16);
pub const ibv_device_cap_flags_IBV_DEVICE_BAD_PKEY_CNTR: ibv_device_cap_flags = ibv_device_cap_flags(2);
pub const ibv_device_cap_flags_IBV_DEVICE_BAD_QKEY_CNTR: ibv_device_cap_flags = ibv_device_cap_flags(4);
pub const ibv_device_cap_flags_IBV_DEVICE_CHANGE_PHY_PORT: ibv_device_cap_flags = ibv_device_cap_flags(32);
pub const ibv_device_cap_flags_IBV_DEVICE_CURR_QP_STATE_MOD: ibv_device_cap_flags = ibv_device_cap_flags(128);
pub const ibv_device_cap_flags_IBV_DEVICE_INIT_TYPE: ibv_device_cap_flags = ibv_device_cap_flags(512);
pub const ibv_device_cap_flags_IBV_DEVICE_MANAGED_FLOW_STEERING: ibv_device_cap_flags = ibv_device_cap_flags(536870912);
pub const ibv_device_cap_flags_IBV_DEVICE_MEM_MGT_EXTENSIONS: ibv_device_cap_flags = ibv_device_cap_flags(2097152);
pub const ibv_device_cap_flags_IBV_DEVICE_MEM_WINDOW: ibv_device_cap_flags = ibv_device_cap_flags(131072);
pub const ibv_device_cap_flags_IBV_DEVICE_MEM_WINDOW_TYPE_2A: ibv_device_cap_flags = ibv_device_cap_flags(8388608);
pub const ibv_device_cap_flags_IBV_DEVICE_MEM_WINDOW_TYPE_2B: ibv_device_cap_flags = ibv_device_cap_flags(16777216);
pub const ibv_device_cap_flags_IBV_DEVICE_N_NOTIFY_CQ: ibv_device_cap_flags = ibv_device_cap_flags(16384);
pub const ibv_device_cap_flags_IBV_DEVICE_PORT_ACTIVE_EVENT: ibv_device_cap_flags = ibv_device_cap_flags(1024);
pub const ibv_device_cap_flags_IBV_DEVICE_RAW_IP_CSUM: ibv_device_cap_flags = ibv_device_cap_flags(67108864);
pub const ibv_device_cap_flags_IBV_DEVICE_RAW_MULTI: ibv_device_cap_flags = ibv_device_cap_flags(8);
pub const ibv_device_cap_flags_IBV_DEVICE_RC_IP_CSUM: ibv_device_cap_flags = ibv_device_cap_flags(33554432);
pub const ibv_device_cap_flags_IBV_DEVICE_RC_RNR_NAK_GEN: ibv_device_cap_flags = ibv_device_cap_flags(4096);
pub const ibv_device_cap_flags_IBV_DEVICE_RESIZE_MAX_WR: ibv_device_cap_flags = ibv_device_cap_flags(1);
pub const ibv_device_cap_flags_IBV_DEVICE_SHUTDOWN_PORT: ibv_device_cap_flags = ibv_device_cap_flags(256);
pub const ibv_device_cap_flags_IBV_DEVICE_SRQ_RESIZE: ibv_device_cap_flags = ibv_device_cap_flags(8192);
pub const ibv_device_cap_flags_IBV_DEVICE_SYS_IMAGE_GUID: ibv_device_cap_flags = ibv_device_cap_flags(2048);
pub const ibv_device_cap_flags_IBV_DEVICE_UD_AV_PORT_ENFORCE: ibv_device_cap_flags = ibv_device_cap_flags(64);
pub const ibv_device_cap_flags_IBV_DEVICE_UD_IP_CSUM: ibv_device_cap_flags = ibv_device_cap_flags(262144);
pub const ibv_device_cap_flags_IBV_DEVICE_XRC: ibv_device_cap_flags = ibv_device_cap_flags(1048576);
pub const ibv_event_type_IBV_EVENT_CLIENT_REREGISTER: ibv_event_type = ibv_event_type(17);
pub const ibv_event_type_IBV_EVENT_COMM_EST: ibv_event_type = ibv_event_type(4);
pub const ibv_event_type_IBV_EVENT_CQ_ERR: ibv_event_type = ibv_event_type(0);
pub const ibv_event_type_IBV_EVENT_DEVICE_FATAL: ibv_event_type = ibv_event_type(8);
pub const ibv_event_type_IBV_EVENT_GID_CHANGE: ibv_event_type = ibv_event_type(18);
pub const ibv_event_type_IBV_EVENT_LID_CHANGE: ibv_event_type = ibv_event_type(11);
pub const ibv_event_type_IBV_EVENT_PATH_MIG: ibv_event_type = ibv_event_type(6);
pub const ibv_event_type_IBV_EVENT_PATH_MIG_ERR: ibv_event_type = ibv_event_type(7);
pub const ibv_event_type_IBV_EVENT_PKEY_CHANGE: ibv_event_type = ibv_event_type(12);
pub const ibv_event_type_IBV_EVENT_PORT_ACTIVE: ibv_event_type = ibv_event_type(9);
pub const ibv_event_type_IBV_EVENT_PORT_ERR: ibv_event_type = ibv_event_type(10);
pub const ibv_event_type_IBV_EVENT_QP_ACCESS_ERR: ibv_event_type = ibv_event_type(3);
pub const ibv_event_type_IBV_EVENT_QP_FATAL: ibv_event_type = ibv_event_type(1);
pub const ibv_event_type_IBV_EVENT_QP_LAST_WQE_REACHED: ibv_event_type = ibv_event_type(16);
pub const ibv_event_type_IBV_EVENT_QP_REQ_ERR: ibv_event_type = ibv_event_type(2);
pub const ibv_event_type_IBV_EVENT_SM_CHANGE: ibv_event_type = ibv_event_type(13);
pub const ibv_event_type_IBV_EVENT_SQ_DRAINED: ibv_event_type = ibv_event_type(5);
pub const ibv_event_type_IBV_EVENT_SRQ_ERR: ibv_event_type = ibv_event_type(14);
pub const ibv_event_type_IBV_EVENT_SRQ_LIMIT_REACHED: ibv_event_type = ibv_event_type(15);
pub const ibv_event_type_IBV_EVENT_WQ_FATAL: ibv_event_type = ibv_event_type(19);
pub const ibv_flow_flags_IBV_FLOW_ATTR_FLAGS_ALLOW_LOOP_BACK: ibv_flow_flags = 1;
pub const ibv_flow_flags_IBV_FLOW_ATTR_FLAGS_DONT_TRAP: ibv_flow_flags = 2;
pub const ibv_ind_table_init_attr_mask_IBV_CREATE_IND_TABLE_RESERVED: ibv_ind_table_init_attr_mask = ibv_ind_table_init_attr_mask(1);
pub const ibv_ops_flags_IBV_OPS_SIGNALED: ibv_ops_flags = ibv_ops_flags(1);
pub const ibv_ops_flags_IBV_OPS_TM_SYNC: ibv_ops_flags = ibv_ops_flags(2);
pub const ibv_port_cap_flags_IBV_PORT_AUTO_MIGR_SUP: ibv_port_cap_flags = ibv_port_cap_flags(32);
pub const ibv_port_cap_flags_IBV_PORT_BOOT_MGMT_SUP: ibv_port_cap_flags = ibv_port_cap_flags(8388608);
pub const ibv_port_cap_flags_IBV_PORT_CAP_MASK_NOTICE_SUP: ibv_port_cap_flags = ibv_port_cap_flags(4194304);
pub const ibv_port_cap_flags_IBV_PORT_CLIENT_REG_SUP: ibv_port_cap_flags = ibv_port_cap_flags(33554432);
pub const ibv_port_cap_flags_IBV_PORT_CM_SUP: ibv_port_cap_flags = ibv_port_cap_flags(65536);
pub const ibv_port_cap_flags_IBV_PORT_DEVICE_MGMT_SUP: ibv_port_cap_flags = ibv_port_cap_flags(524288);
pub const ibv_port_cap_flags_IBV_PORT_DR_NOTICE_SUP: ibv_port_cap_flags = ibv_port_cap_flags(2097152);
pub const ibv_port_cap_flags_IBV_PORT_EXTENDED_SPEEDS_SUP: ibv_port_cap_flags = ibv_port_cap_flags(16384);
pub const ibv_port_cap_flags_IBV_PORT_IP_BASED_GIDS: ibv_port_cap_flags = ibv_port_cap_flags(67108864);
pub const ibv_port_cap_flags_IBV_PORT_LED_INFO_SUP: ibv_port_cap_flags = ibv_port_cap_flags(512);
pub const ibv_port_cap_flags_IBV_PORT_LINK_LATENCY_SUP: ibv_port_cap_flags = ibv_port_cap_flags(16777216);
pub const ibv_port_cap_flags_IBV_PORT_MKEY_NVRAM: ibv_port_cap_flags = ibv_port_cap_flags(128);
pub const ibv_port_cap_flags_IBV_PORT_NOTICE_SUP: ibv_port_cap_flags = ibv_port_cap_flags(4);
pub const ibv_port_cap_flags_IBV_PORT_OPT_IPD_SUP: ibv_port_cap_flags = ibv_port_cap_flags(16);
pub const ibv_port_cap_flags_IBV_PORT_PKEY_NVRAM: ibv_port_cap_flags = ibv_port_cap_flags(256);
pub const ibv_port_cap_flags_IBV_PORT_PKEY_SW_EXT_PORT_TRAP_SUP: ibv_port_cap_flags = ibv_port_cap_flags(4096);
pub const ibv_port_cap_flags_IBV_PORT_REINIT_SUP: ibv_port_cap_flags = ibv_port_cap_flags(262144);
pub const ibv_port_cap_flags_IBV_PORT_SL_MAP_SUP: ibv_port_cap_flags = ibv_port_cap_flags(64);
pub const ibv_port_cap_flags_IBV_PORT_SM: ibv_port_cap_flags = ibv_port_cap_flags(2);
pub const ibv_port_cap_flags_IBV_PORT_SNMP_TUNNEL_SUP: ibv_port_cap_flags = ibv_port_cap_flags(131072);
pub const ibv_port_cap_flags_IBV_PORT_SYS_IMAGE_GUID_SUP: ibv_port_cap_flags = ibv_port_cap_flags(2048);
pub const ibv_port_cap_flags_IBV_PORT_TRAP_SUP: ibv_port_cap_flags = ibv_port_cap_flags(8);
pub const ibv_port_cap_flags_IBV_PORT_VENDOR_CLASS_SUP: ibv_port_cap_flags = ibv_port_cap_flags(1048576);
pub const ibv_qp_open_attr_mask_IBV_QP_OPEN_ATTR_CONTEXT: ibv_qp_open_attr_mask = ibv_qp_open_attr_mask(4);
pub const ibv_qp_open_attr_mask_IBV_QP_OPEN_ATTR_NUM: ibv_qp_open_attr_mask = ibv_qp_open_attr_mask(1);
pub const ibv_qp_open_attr_mask_IBV_QP_OPEN_ATTR_RESERVED: ibv_qp_open_attr_mask = ibv_qp_open_attr_mask(16);
pub const ibv_qp_open_attr_mask_IBV_QP_OPEN_ATTR_TYPE: ibv_qp_open_attr_mask = ibv_qp_open_attr_mask(8);
pub const ibv_qp_open_attr_mask_IBV_QP_OPEN_ATTR_XRCD: ibv_qp_open_attr_mask = ibv_qp_open_attr_mask(2);
pub const ibv_raw_packet_caps_IBV_RAW_PACKET_CAP_CVLAN_STRIPPING: ibv_raw_packet_caps = ibv_raw_packet_caps(1);
pub const ibv_raw_packet_caps_IBV_RAW_PACKET_CAP_DELAY_DROP: ibv_raw_packet_caps = ibv_raw_packet_caps(8);
pub const ibv_raw_packet_caps_IBV_RAW_PACKET_CAP_IP_CSUM: ibv_raw_packet_caps = ibv_raw_packet_caps(4);
pub const ibv_raw_packet_caps_IBV_RAW_PACKET_CAP_SCATTER_FCS: ibv_raw_packet_caps = ibv_raw_packet_caps(2);
pub const ibv_rereg_mr_flags_IBV_REREG_MR_CHANGE_ACCESS: ibv_rereg_mr_flags = ibv_rereg_mr_flags(4);
pub const ibv_rereg_mr_flags_IBV_REREG_MR_CHANGE_PD: ibv_rereg_mr_flags = ibv_rereg_mr_flags(2);
pub const ibv_rereg_mr_flags_IBV_REREG_MR_CHANGE_TRANSLATION: ibv_rereg_mr_flags = ibv_rereg_mr_flags(1);
pub const ibv_rereg_mr_flags_IBV_REREG_MR_FLAGS_SUPPORTED: ibv_rereg_mr_flags = ibv_rereg_mr_flags(15);
pub const ibv_rereg_mr_flags_IBV_REREG_MR_KEEP_VALID: ibv_rereg_mr_flags = ibv_rereg_mr_flags(8);
pub const ibv_rx_hash_fields_IBV_RX_HASH_DST_IPV4: ibv_rx_hash_fields = ibv_rx_hash_fields(2);
pub const ibv_rx_hash_fields_IBV_RX_HASH_DST_IPV6: ibv_rx_hash_fields = ibv_rx_hash_fields(8);
pub const ibv_rx_hash_fields_IBV_RX_HASH_DST_PORT_TCP: ibv_rx_hash_fields = ibv_rx_hash_fields(32);
pub const ibv_rx_hash_fields_IBV_RX_HASH_DST_PORT_UDP: ibv_rx_hash_fields = ibv_rx_hash_fields(128);
pub const ibv_rx_hash_fields_IBV_RX_HASH_INNER: ibv_rx_hash_fields = ibv_rx_hash_fields(2147483648);
pub const ibv_rx_hash_fields_IBV_RX_HASH_SRC_IPV4: ibv_rx_hash_fields = ibv_rx_hash_fields(1);
pub const ibv_rx_hash_fields_IBV_RX_HASH_SRC_IPV6: ibv_rx_hash_fields = ibv_rx_hash_fields(4);
pub const ibv_rx_hash_fields_IBV_RX_HASH_SRC_PORT_TCP: ibv_rx_hash_fields = ibv_rx_hash_fields(16);
pub const ibv_rx_hash_fields_IBV_RX_HASH_SRC_PORT_UDP: ibv_rx_hash_fields = ibv_rx_hash_fields(64);
pub const ibv_rx_hash_function_flags_IBV_RX_HASH_FUNC_TOEPLITZ: ibv_rx_hash_function_flags = ibv_rx_hash_function_flags(1);
pub const ibv_srq_attr_mask_IBV_SRQ_LIMIT: ibv_srq_attr_mask = ibv_srq_attr_mask(2);
pub const ibv_srq_attr_mask_IBV_SRQ_MAX_WR: ibv_srq_attr_mask = ibv_srq_attr_mask(1);
pub const ibv_tm_cap_flags_IBV_TM_CAP_RC: ibv_tm_cap_flags = ibv_tm_cap_flags(1);
pub const ibv_values_mask_IBV_VALUES_MASK_RAW_CLOCK: ibv_values_mask = 1;
pub const ibv_values_mask_IBV_VALUES_MASK_RESERVED: ibv_values_mask = 2;
pub const ibv_wq_attr_mask_IBV_WQ_ATTR_CURR_STATE: ibv_wq_attr_mask = ibv_wq_attr_mask(2);
pub const ibv_wq_attr_mask_IBV_WQ_ATTR_FLAGS: ibv_wq_attr_mask = ibv_wq_attr_mask(4);
pub const ibv_wq_attr_mask_IBV_WQ_ATTR_RESERVED: ibv_wq_attr_mask = ibv_wq_attr_mask(8);
pub const ibv_wq_attr_mask_IBV_WQ_ATTR_STATE: ibv_wq_attr_mask = ibv_wq_attr_mask(1);
pub const ibv_wq_init_attr_mask_IBV_WQ_INIT_ATTR_FLAGS: ibv_wq_init_attr_mask = ibv_wq_init_attr_mask(1);
pub const ibv_wq_init_attr_mask_IBV_WQ_INIT_ATTR_RESERVED: ibv_wq_init_attr_mask = ibv_wq_init_attr_mask(2);
pub const rdma_cm_join_mc_attr_mask_RDMA_CM_JOIN_MC_ATTR_ADDRESS: rdma_cm_join_mc_attr_mask = rdma_cm_join_mc_attr_mask(1);
pub const rdma_cm_join_mc_attr_mask_RDMA_CM_JOIN_MC_ATTR_JOIN_FLAGS: rdma_cm_join_mc_attr_mask = rdma_cm_join_mc_attr_mask(2);
pub const rdma_cm_join_mc_attr_mask_RDMA_CM_JOIN_MC_ATTR_RESERVED: rdma_cm_join_mc_attr_mask = rdma_cm_join_mc_attr_mask(4);
pub const rdma_cm_mc_join_flags_RDMA_MC_JOIN_FLAG_FULLMEMBER: rdma_cm_mc_join_flags = rdma_cm_mc_join_flags(0);
pub const rdma_cm_mc_join_flags_RDMA_MC_JOIN_FLAG_RESERVED: rdma_cm_mc_join_flags = rdma_cm_mc_join_flags(2);
pub const rdma_cm_mc_join_flags_RDMA_MC_JOIN_FLAG_SENDONLY_FULLMEMBER: rdma_cm_mc_join_flags = rdma_cm_mc_join_flags(1);
