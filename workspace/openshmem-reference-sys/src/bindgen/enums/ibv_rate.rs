// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_rate
{
	IBV_RATE_MAX = 0,
	IBV_RATE_2_5_GBPS = 2,
	IBV_RATE_5_GBPS = 5,
	IBV_RATE_10_GBPS = 3,
	IBV_RATE_20_GBPS = 6,
	IBV_RATE_30_GBPS = 4,
	IBV_RATE_40_GBPS = 7,
	IBV_RATE_60_GBPS = 8,
	IBV_RATE_80_GBPS = 9,
	IBV_RATE_120_GBPS = 10,
	IBV_RATE_14_GBPS = 11,
	IBV_RATE_56_GBPS = 12,
	IBV_RATE_112_GBPS = 13,
	IBV_RATE_168_GBPS = 14,
	IBV_RATE_25_GBPS = 15,
	IBV_RATE_100_GBPS = 16,
	IBV_RATE_200_GBPS = 17,
	IBV_RATE_300_GBPS = 18,
}
