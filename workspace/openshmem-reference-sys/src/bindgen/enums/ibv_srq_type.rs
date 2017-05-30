// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_srq_type
{
	IBV_SRQT_BASIC = 0,
	IBV_SRQT_XRC = 1,
}

impl ::core::ops::BitOr<ibv_srq_init_attr_mask> for ibv_srq_init_attr_mask
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		ibv_srq_init_attr_mask(self.0 | other.0)
	}
}
