// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct _bindgen_ty_3(pub u32);

impl BitOr<_bindgen_ty_3> for _bindgen_ty_3
{
	type Output = Self;
	
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		_bindgen_ty_3(self.0 | other.0)
	}
}

impl BitOrAssign for _bindgen_ty_3
{
	
	#[inline(always)]
	fn bitor_assign(&mut self, rhs: _bindgen_ty_3)
	{
		self.0 |= rhs.0;
	}
}

impl BitAnd<_bindgen_ty_3> for _bindgen_ty_3
{
	type Output = Self;
	
	#[inline(always)]
	fn bitand(self, other: Self) -> Self
	{
		_bindgen_ty_3(self.0 & other.0)
	}
}

impl BitAndAssign for _bindgen_ty_3
{
	
	#[inline(always)]
	fn bitand_assign(&mut self, rhs: _bindgen_ty_3)
	{
		self.0 &= rhs.0;
	}
}
