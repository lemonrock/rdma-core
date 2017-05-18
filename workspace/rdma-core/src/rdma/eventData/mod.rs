// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::queuePairs::QueuePairNumber;
use ::rdma_core_sys::*;
use ::rust_extra::unlikely;
use ::std::slice::from_raw_parts;


include!("ConnectionEventData.rs");
include!("EventData.rs");
include!("EstablishedConnectionEventData.rs");
include!("RequestedConnectionEventData.rs");
include!("UnreliableDatagramEventData.rs");