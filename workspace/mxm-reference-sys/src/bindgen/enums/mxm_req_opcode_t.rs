// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum mxm_req_opcode_t
{
	MXM_REQ_OP_SEND = 0,
	MXM_REQ_OP_SEND_SYNC = 1,
	MXM_REQ_OP_PUT = 2,
	MXM_REQ_OP_PUT_SYNC = 3,
	MXM_REQ_OP_GET = 4,
	MXM_REQ_OP_AM = 5,
	MXM_REQ_OP_ATOMIC_ADD = 6,
	MXM_REQ_OP_ATOMIC_FADD = 7,
	MXM_REQ_OP_ATOMIC_SWAP = 8,
	MXM_REQ_OP_ATOMIC_CSWAP = 9,
	MXM_REQ_OP_LAST = 10,
}
