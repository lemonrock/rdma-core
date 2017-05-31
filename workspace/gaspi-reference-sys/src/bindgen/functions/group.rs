// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_group_add(group: gaspi_group_t, rank: gaspi_rank_t) -> gaspi_return_t;
	pub fn gaspi_group_commit(group: gaspi_group_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_group_create(group: *mut gaspi_group_t) -> gaspi_return_t;
	pub fn gaspi_group_delete(group: gaspi_group_t) -> gaspi_return_t;
	pub fn gaspi_group_max(group_max: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_group_num(group_num: *mut gaspi_number_t) -> gaspi_return_t;
	pub fn gaspi_group_ranks(group: gaspi_group_t, group_ranks: *mut gaspi_rank_t) -> gaspi_return_t;
	pub fn gaspi_group_size(group: gaspi_group_t, group_size: *mut gaspi_number_t) -> gaspi_return_t;
}
