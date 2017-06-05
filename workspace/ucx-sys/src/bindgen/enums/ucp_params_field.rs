// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ucp_params_field
{
	UCP_PARAM_FIELD_FEATURES = 1,
	UCP_PARAM_FIELD_REQUEST_SIZE = 2,
	UCP_PARAM_FIELD_REQUEST_INIT = 4,
	UCP_PARAM_FIELD_REQUEST_CLEANUP = 8,
	UCP_PARAM_FIELD_TAG_SENDER_MASK = 16,
	UCP_PARAM_FIELD_MT_WORKERS_SHARED = 32,
	UCP_PARAM_FIELD_ESTIMATED_NUM_EPS = 64,
}
