// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum uct_event_types
{
	UCP_EVENT_TX_COMPLETION = 1,
	UCP_EVENT_TX_RESOURCES = 2,
	UCP_EVENT_RX_COMPLETION = 4,
	UCP_EVENT_RX_RESOURCES = 8,
	UCP_EVENT_TX_ERROR = 16,
	UCP_EVENT_RX_ERROR = 32,
}
