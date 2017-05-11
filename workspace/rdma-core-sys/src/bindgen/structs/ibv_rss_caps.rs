// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct ibv_rss_caps
{
	pub supported_qpts: u32,
	pub max_rwq_indirection_tables: u32,
	pub max_rwq_indirection_table_size: u32,
	pub rx_hash_fields_mask: u64,
	pub rx_hash_function: u8,
}

impl Clone for ibv_rss_caps
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}
