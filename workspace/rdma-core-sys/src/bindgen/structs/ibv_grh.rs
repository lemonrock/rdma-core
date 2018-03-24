// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_grh
{
	pub version_tclass_flow: __be32,
	pub paylen: __be16,
	pub next_hdr: u8,
	pub hop_limit: u8,
	pub sgid: ibv_gid,
	pub dgid: ibv_gid,
}

impl Default for ibv_grh
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_grh
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_grh {{ sgid: {:?}, dgid: {:?} }}", self.sgid, self.dgid)
	}
}
