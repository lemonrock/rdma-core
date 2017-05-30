// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_flow_spec_type
{
	IBV_FLOW_SPEC_ETH = 32,
	IBV_FLOW_SPEC_IPV4 = 48,
	IBV_FLOW_SPEC_IPV6 = 49,
	IBV_FLOW_SPEC_IPV4_EXT = 50,
	IBV_FLOW_SPEC_TCP = 64,
	IBV_FLOW_SPEC_UDP = 65,
	IBV_FLOW_SPEC_VXLAN_TUNNEL = 80,
	IBV_FLOW_SPEC_INNER = 256,
	IBV_FLOW_SPEC_ACTION_TAG = 4096,
}
