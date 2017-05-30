// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_node_type
{
	IBV_NODE_UNKNOWN = -1,
	IBV_NODE_CA = 1,
	IBV_NODE_SWITCH = 2,
	IBV_NODE_ROUTER = 3,
	IBV_NODE_RNIC = 4,
	IBV_NODE_USNIC = 5,
	IBV_NODE_USNIC_UDP = 6,
}
