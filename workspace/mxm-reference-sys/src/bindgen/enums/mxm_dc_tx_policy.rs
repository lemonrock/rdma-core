// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_dc_tx_policy
{
	MXM_DC_TX_POLICY_RANDOM = 0,
	MXM_DC_TX_POLICY_LRU = 1,
	MXM_DC_TX_POLICY_HASH_CONN = 2,
	MXM_DC_TX_POLICY_HASH_DLID = 3,
	MXM_DC_TX_POLICY_DCS = 4,
	MXM_DC_TX_POLICY_LAST = 5,
}
