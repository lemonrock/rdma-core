// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_segment_alloc(segment_id: gaspi_segment_id_t, size: gaspi_size_t, alloc_policy: gaspi_alloc_t) -> gaspi_return_t;
	pub fn gaspi_segment_bind(segment_id: gaspi_segment_id_t, pointer: gaspi_pointer_t, size: gaspi_size_t, memory_description: gaspi_memory_description_t) -> gaspi_return_t;
	pub fn gaspi_segment_create(segment_id: gaspi_segment_id_t, size: gaspi_size_t, group: gaspi_group_t, timeout: gaspi_timeout_t, alloc_policy: gaspi_alloc_t) -> gaspi_return_t;
	pub fn gaspi_segment_delete(segment_id: gaspi_segment_id_t) -> gaspi_return_t;
	pub fn gaspi_segment_list(num: gaspi_number_t, segment_id_list: *mut gaspi_segment_id_t) -> gaspi_return_t;
	pub fn gaspi_segment_max(segment_max: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_segment_num(segment_num: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_segment_ptr(segment_id: gaspi_segment_id_t, pointer: *mut gaspi_pointer_t) -> gaspi_return_t;
	pub fn gaspi_segment_register(segment_id: gaspi_segment_id_t, rank: gaspi_rank_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_segment_use(segment_id: gaspi_segment_id_t, pointer: gaspi_pointer_t, size: gaspi_size_t, group: gaspi_group_t, timeout: gaspi_timeout_t, memory_description: gaspi_memory_description_t) -> gaspi_return_t;
}
