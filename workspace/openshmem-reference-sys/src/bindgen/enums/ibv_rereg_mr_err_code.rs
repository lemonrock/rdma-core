// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_rereg_mr_err_code
{
	IBV_REREG_MR_ERR_INPUT = -1,
	IBV_REREG_MR_ERR_DONT_FORK_NEW = -2,
	IBV_REREG_MR_ERR_DO_FORK_OLD = -3,
	IBV_REREG_MR_ERR_CMD = -4,
	IBV_REREG_MR_ERR_CMD_AND_DO_FORK_NEW = -5,
}
