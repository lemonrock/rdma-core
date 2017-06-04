// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_stats_aggregate_mode
{
	MXM_STATS_AGGREGATE_SUM = 0,
	MXM_STATS_AGGREGATE_AVERAGE = 1,
	MXM_STATS_AGGREGATE_MIN = 2,
	MXM_STATS_AGGREGATE_MAX = 3,
	MXM_STATS_AGGREGATE_COUNT_NZ = 4,
	MXM_STATS_AGGREGATE_LAST = 5,
}
