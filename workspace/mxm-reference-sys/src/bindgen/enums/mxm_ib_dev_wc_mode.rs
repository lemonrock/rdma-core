// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_ib_dev_wc_mode
{
	MXM_IB_DEV_WC_FOR_WQE = 0,
	MXM_IB_DEV_WC_FOR_DB = 1,
	MXM_IB_DEV_WC_FORCE_FLUSH = 2,
	MXM_IB_DEV_WC_MODE_LAST = 3,
}
