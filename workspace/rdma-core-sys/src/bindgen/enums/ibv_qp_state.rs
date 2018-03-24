// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_qp_state
{
	IBV_QPS_RESET = 0,
	IBV_QPS_INIT = 1,
	IBV_QPS_RTR = 2,
	IBV_QPS_RTS = 3,
	IBV_QPS_SQD = 4,
	IBV_QPS_SQE = 5,
	IBV_QPS_ERR = 6,
	IBV_QPS_UNKNOWN = 7,
}
