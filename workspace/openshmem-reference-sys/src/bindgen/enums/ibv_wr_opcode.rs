// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_wr_opcode
{
	IBV_WR_RDMA_WRITE = 0,
	IBV_WR_RDMA_WRITE_WITH_IMM = 1,
	IBV_WR_SEND = 2,
	IBV_WR_SEND_WITH_IMM = 3,
	IBV_WR_RDMA_READ = 4,
	IBV_WR_ATOMIC_CMP_AND_SWP = 5,
	IBV_WR_ATOMIC_FETCH_AND_ADD = 6,
	IBV_WR_LOCAL_INV = 7,
	IBV_WR_BIND_MW = 8,
	IBV_WR_SEND_WITH_INV = 9,
	IBV_WR_TSO = 10,
}

impl ::core::ops::BitOr<ibv_send_flags> for ibv_send_flags
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		ibv_send_flags(self.0 | other.0)
	}
}
