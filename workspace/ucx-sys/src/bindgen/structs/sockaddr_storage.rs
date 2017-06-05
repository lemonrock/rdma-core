// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct sockaddr_storage
{
	pub ss_family: sa_family_t,
	pub __ss_padding: [c_char; 118usize],
	pub __ss_align: c_ulong,
}

impl Default for sockaddr_storage
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
