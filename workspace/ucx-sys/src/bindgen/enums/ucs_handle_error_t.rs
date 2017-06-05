// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucs_handle_error_t
{
	UCS_HANDLE_ERROR_BACKTRACE = 0,
	UCS_HANDLE_ERROR_FREEZE = 1,
	UCS_HANDLE_ERROR_DEBUG = 2,
	UCS_HANDLE_ERROR_LAST = 3,
}

impl ::core::ops::BitOr<ucs_config_print_flags_t> for ucs_config_print_flags_t
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		ucs_config_print_flags_t(self.0 | other.0)
	}
}

impl ::core::ops::BitOrAssign for ucs_config_print_flags_t
{
	fn bitor_assign(&mut self, rhs: ucs_config_print_flags_t)
	{
		self.0 |= rhs.0;
	}
}

impl ::core::ops::BitAnd<ucs_config_print_flags_t> for ucs_config_print_flags_t
{
	type Output = Self;
	fn bitand(self, other: Self) -> Self
	{
		ucs_config_print_flags_t(self.0 & other.0)
	}
}

impl ::core::ops::BitAndAssign for ucs_config_print_flags_t
{
	fn bitand_assign(&mut self, rhs: ucs_config_print_flags_t)
	{
		self.0 &= rhs.0;
	}
}
