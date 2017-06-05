// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum uct_progress_types
{
	UCT_PROGRESS_RX = 1,
	UCT_PROGRESS_TX = 2,
}

impl ::core::ops::BitOr<uct_am_cb_cap> for uct_am_cb_cap
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		uct_am_cb_cap(self.0 | other.0)
	}
}

impl ::core::ops::BitOrAssign for uct_am_cb_cap
{
	fn bitor_assign(&mut self, rhs: uct_am_cb_cap)
	{
		self.0 |= rhs.0;
	}
}

impl ::core::ops::BitAnd<uct_am_cb_cap> for uct_am_cb_cap
{
	type Output = Self;
	fn bitand(self, other: Self) -> Self
	{
		uct_am_cb_cap(self.0 & other.0)
	}
}

impl ::core::ops::BitAndAssign for uct_am_cb_cap
{
	fn bitand_assign(&mut self, rhs: uct_am_cb_cap)
	{
		self.0 &= rhs.0;
	}
}
