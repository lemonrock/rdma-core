// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
pub struct ibv_device
{
	pub _ops: _ibv_device_ops,
	pub node_type: ibv_node_type,
	pub transport_type: ibv_transport_type,
	pub name: [c_char; 64usize],
	pub dev_name: [c_char; 64usize],
	pub dev_path: [c_char; 256usize],
	pub ibdev_path: [c_char; 256usize],
}

impl Default for ibv_device
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
