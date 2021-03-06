// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[allow(dead_code)]
pub struct ReceiveWorkQueueIndirectionTable<'a>
{
	pub(crate) pointer: *mut ibv_rwq_ind_table,
	pub(crate) context: &'a Context,
	pub(crate) indirectionTable: Vec<*mut ibv_wq>,
}

impl<'a> Drop for ReceiveWorkQueueIndirectionTable<'a>
{
	#[inline(always)]
	fn drop(&mut self)
	{
		panic_on_errno!(rust_ibv_destroy_rwq_ind_table, self.pointer)
	}
}
