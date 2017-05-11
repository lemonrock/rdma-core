// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_rx_hash_fields
{
	IBV_RX_HASH_SRC_IPV4 = 1,
	IBV_RX_HASH_DST_IPV4 = 2,
	IBV_RX_HASH_SRC_IPV6 = 4,
	IBV_RX_HASH_DST_IPV6 = 8,
	IBV_RX_HASH_SRC_PORT_TCP = 16,
	IBV_RX_HASH_DST_PORT_TCP = 32,
	IBV_RX_HASH_SRC_PORT_UDP = 64,
	IBV_RX_HASH_DST_PORT_UDP = 128,
}
