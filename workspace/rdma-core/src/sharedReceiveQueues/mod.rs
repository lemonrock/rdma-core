// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::Context;
use ::ExtendedReliableConnectionDomain;
use ::ProtectionDomain;
use ::completionQueues::CompletionQueue;
use ::libc::c_int;
use ::rdma_core_sys::*;
use ::std::mem::uninitialized;


include!("ExtendedSharedReceiveQueue.rs");
include!("SharedReceiveQueue.rs");
include!("SharedReceiveQueueNumber.rs");
include!("SharedReceiveQueueSettings.rs");
include!("UnextendedSharedReceiveQueue.rs");