// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_wc_status
{
	IBV_WC_SUCCESS = 0,
	IBV_WC_LOC_LEN_ERR = 1,
	IBV_WC_LOC_QP_OP_ERR = 2,
	IBV_WC_LOC_EEC_OP_ERR = 3,
	IBV_WC_LOC_PROT_ERR = 4,
	IBV_WC_WR_FLUSH_ERR = 5,
	IBV_WC_MW_BIND_ERR = 6,
	IBV_WC_BAD_RESP_ERR = 7,
	IBV_WC_LOC_ACCESS_ERR = 8,
	IBV_WC_REM_INV_REQ_ERR = 9,
	IBV_WC_REM_ACCESS_ERR = 10,
	IBV_WC_REM_OP_ERR = 11,
	IBV_WC_RETRY_EXC_ERR = 12,
	IBV_WC_RNR_RETRY_EXC_ERR = 13,
	IBV_WC_LOC_RDD_VIOL_ERR = 14,
	IBV_WC_REM_INV_RD_REQ_ERR = 15,
	IBV_WC_REM_ABORT_ERR = 16,
	IBV_WC_INV_EECN_ERR = 17,
	IBV_WC_INV_EEC_STATE_ERR = 18,
	IBV_WC_FATAL_ERR = 19,
	IBV_WC_RESP_TIMEOUT_ERR = 20,
	IBV_WC_GENERAL_ERR = 21,
	IBV_WC_TM_ERR = 22,
	IBV_WC_TM_RNDV_INCOMPLETE = 23,
}
