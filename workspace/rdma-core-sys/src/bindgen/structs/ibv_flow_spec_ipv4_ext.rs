// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_flow_spec_ipv4_ext
{
	pub type_: ibv_flow_spec_type,
	pub size: u16,
	pub val: ibv_flow_ipv4_ext_filter,
	pub mask: ibv_flow_ipv4_ext_filter,
}

impl Default for ibv_flow_spec_ipv4_ext
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}

impl Debug for ibv_flow_spec_ipv4_ext
{
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter) -> Result
	{
		write!(f, "ibv_flow_spec_ipv4_ext {{ type: {:?}, val: {:?}, mask: {:?} }}", self.type_, self.val, self.mask)
	}
}
