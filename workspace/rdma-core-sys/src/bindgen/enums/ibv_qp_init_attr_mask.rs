// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_qp_init_attr_mask
{
	IBV_QP_INIT_ATTR_PD = 1,
	IBV_QP_INIT_ATTR_XRCD = 2,
	IBV_QP_INIT_ATTR_CREATE_FLAGS = 4,
	IBV_QP_INIT_ATTR_MAX_TSO_HEADER = 8,
	IBV_QP_INIT_ATTR_IND_TABLE = 16,
	IBV_QP_INIT_ATTR_RX_HASH = 32,
	IBV_QP_INIT_ATTR_RESERVED = 64,
}
