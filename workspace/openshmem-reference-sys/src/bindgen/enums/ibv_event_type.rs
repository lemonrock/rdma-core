// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ibv_event_type
{
	IBV_EVENT_CQ_ERR = 0,
	IBV_EVENT_QP_FATAL = 1,
	IBV_EVENT_QP_REQ_ERR = 2,
	IBV_EVENT_QP_ACCESS_ERR = 3,
	IBV_EVENT_COMM_EST = 4,
	IBV_EVENT_SQ_DRAINED = 5,
	IBV_EVENT_PATH_MIG = 6,
	IBV_EVENT_PATH_MIG_ERR = 7,
	IBV_EVENT_DEVICE_FATAL = 8,
	IBV_EVENT_PORT_ACTIVE = 9,
	IBV_EVENT_PORT_ERR = 10,
	IBV_EVENT_LID_CHANGE = 11,
	IBV_EVENT_PKEY_CHANGE = 12,
	IBV_EVENT_SM_CHANGE = 13,
	IBV_EVENT_SRQ_ERR = 14,
	IBV_EVENT_SRQ_LIMIT_REACHED = 15,
	IBV_EVENT_QP_LAST_WQE_REACHED = 16,
	IBV_EVENT_CLIENT_REREGISTER = 17,
	IBV_EVENT_GID_CHANGE = 18,
	IBV_EVENT_WQ_FATAL = 19,
}
