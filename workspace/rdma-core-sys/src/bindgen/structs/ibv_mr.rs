// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_mr
{
	pub context: *mut ibv_context,
	pub pd: *mut ibv_pd,
	pub addr: *mut c_void,
	pub length: usize,
	pub handle: u32,
	pub lkey: u32,
	pub rkey: u32,
}

impl Default for ibv_mr
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_mr
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_mr {{ context: {:?}, pd: {:?}, addr: {:?} }}", self.context, self.pd, self.addr)
	}
}
