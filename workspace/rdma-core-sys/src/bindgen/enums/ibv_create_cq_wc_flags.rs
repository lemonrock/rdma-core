// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_create_cq_wc_flags
{
	IBV_WC_EX_WITH_BYTE_LEN = 1,
	IBV_WC_EX_WITH_IMM = 2,
	IBV_WC_EX_WITH_QP_NUM = 4,
	IBV_WC_EX_WITH_SRC_QP = 8,
	IBV_WC_EX_WITH_SLID = 16,
	IBV_WC_EX_WITH_SL = 32,
	IBV_WC_EX_WITH_DLID_PATH_BITS = 64,
	IBV_WC_EX_WITH_COMPLETION_TIMESTAMP = 128,
}

impl ::core::ops::BitOr<ibv_wc_flags> for ibv_wc_flags
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		ibv_wc_flags(self.0 | other.0)
	}
}
