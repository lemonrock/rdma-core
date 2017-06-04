// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_req_state_t
{
	MXM_REQ_NEW = 1,
	MXM_REQ_EXPECTED = 2,
	MXM_REQ_INPROGRESS = 4,
	MXM_REQ_SENT = 8,
	MXM_REQ_READY = 16,
	MXM_REQ_COMPLETED = 32,
}
