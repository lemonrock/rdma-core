// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn gaspi_proc_init(timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_proc_kill(rank: gaspi_rank_t, timeout: gaspi_timeout_t) -> gaspi_return_t;
	pub fn gaspi_proc_num(proc_num: *mut gaspi_rank_t) -> gaspi_return_t;
	pub fn gaspi_proc_rank(rank: *mut gaspi_rank_t) -> gaspi_return_t;
	pub fn gaspi_proc_term(timeout: gaspi_timeout_t) -> gaspi_return_t;
}
