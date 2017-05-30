// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum rdma_cm_event_type
{
	RDMA_CM_EVENT_ADDR_RESOLVED = 0,
	RDMA_CM_EVENT_ADDR_ERROR = 1,
	RDMA_CM_EVENT_ROUTE_RESOLVED = 2,
	RDMA_CM_EVENT_ROUTE_ERROR = 3,
	RDMA_CM_EVENT_CONNECT_REQUEST = 4,
	RDMA_CM_EVENT_CONNECT_RESPONSE = 5,
	RDMA_CM_EVENT_CONNECT_ERROR = 6,
	RDMA_CM_EVENT_UNREACHABLE = 7,
	RDMA_CM_EVENT_REJECTED = 8,
	RDMA_CM_EVENT_ESTABLISHED = 9,
	RDMA_CM_EVENT_DISCONNECTED = 10,
	RDMA_CM_EVENT_DEVICE_REMOVAL = 11,
	RDMA_CM_EVENT_MULTICAST_JOIN = 12,
	RDMA_CM_EVENT_MULTICAST_ERROR = 13,
	RDMA_CM_EVENT_ADDR_CHANGE = 14,
	RDMA_CM_EVENT_TIMEWAIT_EXIT = 15,
}
