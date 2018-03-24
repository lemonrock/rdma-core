// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub const ibv_wc_flags_IBV_WC_GRH: ibv_wc_flags = ibv_wc_flags(1);
pub const ibv_wc_flags_IBV_WC_IP_CSUM_OK: ibv_wc_flags = ibv_wc_flags(4);
pub const ibv_wc_flags_IBV_WC_TM_DATA_VALID: ibv_wc_flags = ibv_wc_flags(64);
pub const ibv_wc_flags_IBV_WC_TM_MATCH: ibv_wc_flags = ibv_wc_flags(32);
pub const ibv_wc_flags_IBV_WC_TM_SYNC_REQ: ibv_wc_flags = ibv_wc_flags(16);
pub const ibv_wc_flags_IBV_WC_WITH_IMM: ibv_wc_flags = ibv_wc_flags(2);
pub const ibv_wc_flags_IBV_WC_WITH_INV: ibv_wc_flags = ibv_wc_flags(8);
