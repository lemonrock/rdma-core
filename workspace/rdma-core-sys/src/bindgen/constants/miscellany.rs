// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub const ibv_access_flags_IBV_ACCESS_LOCAL_WRITE: ibv_access_flags = ibv_access_flags(1);
pub const ibv_access_flags_IBV_ACCESS_MW_BIND: ibv_access_flags = ibv_access_flags(16);
pub const ibv_access_flags_IBV_ACCESS_ON_DEMAND: ibv_access_flags = ibv_access_flags(64);
pub const ibv_access_flags_IBV_ACCESS_REMOTE_ATOMIC: ibv_access_flags = ibv_access_flags(8);
pub const ibv_access_flags_IBV_ACCESS_REMOTE_READ: ibv_access_flags = ibv_access_flags(4);
pub const ibv_access_flags_IBV_ACCESS_REMOTE_WRITE: ibv_access_flags = ibv_access_flags(2);
pub const ibv_access_flags_IBV_ACCESS_ZERO_BASED: ibv_access_flags = ibv_access_flags(32);
pub const ibv_qp_attr_mask_IBV_QP_ACCESS_FLAGS: ibv_qp_attr_mask = ibv_qp_attr_mask(8);
pub const ibv_qp_attr_mask_IBV_QP_ALT_PATH: ibv_qp_attr_mask = ibv_qp_attr_mask(16384);
pub const ibv_qp_attr_mask_IBV_QP_AV: ibv_qp_attr_mask = ibv_qp_attr_mask(128);
pub const ibv_qp_attr_mask_IBV_QP_CAP: ibv_qp_attr_mask = ibv_qp_attr_mask(524288);
pub const ibv_qp_attr_mask_IBV_QP_CUR_STATE: ibv_qp_attr_mask = ibv_qp_attr_mask(2);
pub const ibv_qp_attr_mask_IBV_QP_DEST_QPN: ibv_qp_attr_mask = ibv_qp_attr_mask(1048576);
pub const ibv_qp_attr_mask_IBV_QP_EN_SQD_ASYNC_NOTIFY: ibv_qp_attr_mask = ibv_qp_attr_mask(4);
pub const ibv_qp_attr_mask_IBV_QP_MAX_DEST_RD_ATOMIC: ibv_qp_attr_mask = ibv_qp_attr_mask(131072);
pub const ibv_qp_attr_mask_IBV_QP_MAX_QP_RD_ATOMIC: ibv_qp_attr_mask = ibv_qp_attr_mask(8192);
pub const ibv_qp_attr_mask_IBV_QP_MIN_RNR_TIMER: ibv_qp_attr_mask = ibv_qp_attr_mask(32768);
pub const ibv_qp_attr_mask_IBV_QP_PATH_MIG_STATE: ibv_qp_attr_mask = ibv_qp_attr_mask(262144);
pub const ibv_qp_attr_mask_IBV_QP_PATH_MTU: ibv_qp_attr_mask = ibv_qp_attr_mask(256);
pub const ibv_qp_attr_mask_IBV_QP_PKEY_INDEX: ibv_qp_attr_mask = ibv_qp_attr_mask(16);
pub const ibv_qp_attr_mask_IBV_QP_PORT: ibv_qp_attr_mask = ibv_qp_attr_mask(32);
pub const ibv_qp_attr_mask_IBV_QP_QKEY: ibv_qp_attr_mask = ibv_qp_attr_mask(64);
pub const ibv_qp_attr_mask_IBV_QP_RATE_LIMIT: ibv_qp_attr_mask = ibv_qp_attr_mask(33554432);
pub const ibv_qp_attr_mask_IBV_QP_RETRY_CNT: ibv_qp_attr_mask = ibv_qp_attr_mask(1024);
pub const ibv_qp_attr_mask_IBV_QP_RNR_RETRY: ibv_qp_attr_mask = ibv_qp_attr_mask(2048);
pub const ibv_qp_attr_mask_IBV_QP_RQ_PSN: ibv_qp_attr_mask = ibv_qp_attr_mask(4096);
pub const ibv_qp_attr_mask_IBV_QP_SQ_PSN: ibv_qp_attr_mask = ibv_qp_attr_mask(65536);
pub const ibv_qp_attr_mask_IBV_QP_STATE: ibv_qp_attr_mask = ibv_qp_attr_mask(1);
pub const ibv_qp_attr_mask_IBV_QP_TIMEOUT: ibv_qp_attr_mask = ibv_qp_attr_mask(512);
pub const ibv_send_flags_IBV_SEND_FENCE: ibv_send_flags = ibv_send_flags(1);
pub const ibv_send_flags_IBV_SEND_INLINE: ibv_send_flags = ibv_send_flags(8);
pub const ibv_send_flags_IBV_SEND_IP_CSUM: ibv_send_flags = ibv_send_flags(16);
pub const ibv_send_flags_IBV_SEND_SIGNALED: ibv_send_flags = ibv_send_flags(2);
pub const ibv_send_flags_IBV_SEND_SOLICITED: ibv_send_flags = ibv_send_flags(4);
pub const ibv_wc_flags_IBV_WC_GRH: ibv_wc_flags = ibv_wc_flags(1);
pub const ibv_wc_flags_IBV_WC_IP_CSUM_OK: ibv_wc_flags = ibv_wc_flags(4);
pub const ibv_wc_flags_IBV_WC_WITH_IMM: ibv_wc_flags = ibv_wc_flags(2);
pub const ibv_wc_flags_IBV_WC_WITH_INV: ibv_wc_flags = ibv_wc_flags(8);
