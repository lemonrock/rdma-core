// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct gaspi_config_t
{
	pub group_max: gaspi_number_t,
	pub segment_max: gaspi_number_t,
	pub queue_num: gaspi_number_t,
	pub queue_size_max: gaspi_number_t,
	pub transfer_size_max: gaspi_size_t,
	pub notification_num: gaspi_number_t,
	pub passive_queue_size_max: gaspi_number_t,
	pub passive_transfer_size_max: gaspi_size_t,
	pub allreduce_buf_size: gaspi_size_t,
	pub allreduce_elem_max: gaspi_number_t,
	pub network: gaspi_network_t,
	pub build_infrastructure: gaspi_number_t,
	pub user_defined: *mut c_void,
}

impl Clone for gaspi_config_t
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}

impl Default for gaspi_config_t
{
	#[inline(always)]
	fn default() -> Self
	{
		unsafe { zeroed() }
	}
}
