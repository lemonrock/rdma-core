// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct sched_param
{
	pub sched_priority: c_int,
	pub sched_ss_low_priority: c_int,
	pub sched_ss_repl_period: timespec,
	pub sched_ss_init_budget: timespec,
	pub sched_ss_max_repl: c_int,
}

impl Clone for sched_param
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		*self
	}
}
