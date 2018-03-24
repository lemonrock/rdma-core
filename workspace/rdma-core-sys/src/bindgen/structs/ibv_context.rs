// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_context
{
	pub device: *mut ibv_device,
	pub ops: ibv_context_ops,
	pub cmd_fd: c_int,
	pub async_fd: c_int,
	pub num_comp_vectors: c_int,
	pub mutex: pthread_mutex_t,
	pub abi_compat: *mut c_void,
}

impl Default for ibv_context
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_context
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_context {{ device: {:?}, ops: {:?}, abi_compat: {:?} }}", self.device, self.ops, self.abi_compat)
	}
}
