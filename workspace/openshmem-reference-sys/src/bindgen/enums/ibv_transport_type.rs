// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_transport_type
{
	IBV_TRANSPORT_UNKNOWN = -1,
	IBV_TRANSPORT_IB = 0,
	IBV_TRANSPORT_IWARP = 1,
	IBV_TRANSPORT_USNIC = 2,
	IBV_TRANSPORT_USNIC_UDP = 3,
}
