// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_port_state
{
	IBV_PORT_NOP = 0,
	IBV_PORT_DOWN = 1,
	IBV_PORT_INIT = 2,
	IBV_PORT_ARMED = 3,
	IBV_PORT_ACTIVE = 4,
	IBV_PORT_ACTIVE_DEFER = 5,
}
