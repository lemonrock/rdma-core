// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_wc_opcode
{
	IBV_WC_SEND = 0,
	IBV_WC_RDMA_WRITE = 1,
	IBV_WC_RDMA_READ = 2,
	IBV_WC_COMP_SWAP = 3,
	IBV_WC_FETCH_ADD = 4,
	IBV_WC_BIND_MW = 5,
	IBV_WC_LOCAL_INV = 6,
	IBV_WC_TSO = 7,
	IBV_WC_RECV = 128,
	IBV_WC_RECV_RDMA_WITH_IMM = 129,
	IBV_WC_TM_ADD = 130,
	IBV_WC_TM_DEL = 131,
	IBV_WC_TM_SYNC = 132,
	IBV_WC_TM_RECV = 133,
	IBV_WC_TM_NO_TAG = 134,
}
