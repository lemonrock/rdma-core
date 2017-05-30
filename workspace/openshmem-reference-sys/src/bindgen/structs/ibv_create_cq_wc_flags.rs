// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ibv_create_cq_wc_flags(pub c_uint);

impl ::core::ops::BitOr<_bindgen_ty_3> for _bindgen_ty_3
{
	type Output = Self;
	#[inline(always)]
	fn bitor(self, other: Self) -> Self
	{
		_bindgen_ty_3(self.0 | other.0)
	}
}
