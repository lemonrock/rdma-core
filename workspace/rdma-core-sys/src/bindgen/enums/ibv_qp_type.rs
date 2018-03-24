// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_qp_type
{
	IBV_QPT_RC = 2,
	IBV_QPT_UC = 3,
	IBV_QPT_UD = 4,
	IBV_QPT_RAW_PACKET = 8,
	IBV_QPT_XRC_SEND = 9,
	IBV_QPT_XRC_RECV = 10,
	IBV_QPT_DRIVER = 255,
}
