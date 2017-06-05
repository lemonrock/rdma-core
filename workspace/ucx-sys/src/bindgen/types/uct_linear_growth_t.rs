// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub type uct_linear_growth_t = uct_linear_growth;

impl ::core::ops::BitOr<_bindgen_ty_1> for _bindgen_ty_1
{
	type Output = Self;
	fn bitor(self, other: Self) -> Self
	{
		_bindgen_ty_1(self.0 | other.0)
	}
}

impl ::core::ops::BitOrAssign for _bindgen_ty_1
{
	fn bitor_assign(&mut self, rhs: _bindgen_ty_1)
	{
		self.0 |= rhs.0;
	}
}

impl ::core::ops::BitAnd<_bindgen_ty_1> for _bindgen_ty_1
{
	type Output = Self;
	fn bitand(self, other: Self) -> Self
	{
		_bindgen_ty_1(self.0 & other.0)
	}
}

impl ::core::ops::BitAndAssign for _bindgen_ty_1
{
	fn bitand_assign(&mut self, rhs: _bindgen_ty_1)
	{
		self.0 &= rhs.0;
	}
}
