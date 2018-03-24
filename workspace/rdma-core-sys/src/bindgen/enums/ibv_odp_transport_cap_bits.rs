// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_odp_transport_cap_bits
{
	IBV_ODP_SUPPORT_SEND = 1,
	IBV_ODP_SUPPORT_RECV = 2,
	IBV_ODP_SUPPORT_WRITE = 4,
	IBV_ODP_SUPPORT_READ = 8,
	IBV_ODP_SUPPORT_ATOMIC = 16,
}
