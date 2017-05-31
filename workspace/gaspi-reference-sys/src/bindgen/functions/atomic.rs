// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_atomic_compare_swap(segment_id: gaspi_segment_id_t, offset: gaspi_offset_t, rank: gaspi_rank_t, comparator: gaspi_atomic_value_t, value_new: gaspi_atomic_value_t, value_old: *mut gaspi_atomic_value_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_atomic_fetch_add(segment_id: gaspi_segment_id_t, offset: gaspi_offset_t, rank: gaspi_rank_t, value_add: gaspi_atomic_value_t, value_old: *mut gaspi_atomic_value_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_atomic_max(max_value: *mut gaspi_atomic_value_t) -> gaspi_return_t;
}
