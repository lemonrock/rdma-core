// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_rereg_mr_flags
{
	IBV_REREG_MR_CHANGE_TRANSLATION = 1,
	IBV_REREG_MR_CHANGE_PD = 2,
	IBV_REREG_MR_CHANGE_ACCESS = 4,
	IBV_REREG_MR_KEEP_VALID = 8,
	IBV_REREG_MR_FLAGS_SUPPORTED = 15,
}
