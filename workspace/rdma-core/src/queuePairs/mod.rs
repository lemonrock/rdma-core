// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::LocalIdentifier;
use ::ProtectionDomain;
use ::completionQueues::*;
use ::sharedReceiveQueues::*;
use ::rdma_core_sys::*;
use ::rust_extra::likely;
use ::std::cmp::Eq;
use ::std::cmp::PartialEq;
use ::std::collections::HashSet;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::zeroed;


include!("AttributeFlags.rs");
include!("MultiCastGroupIdentifier.rs");
include!("QueuePair.rs");
include!("QueuePairNumber.rs");
include!("UnextendedQueuePair.rs");

include!("ExtendedReliableConnectionReceiveQueuePair.rs");
include!("ExtendedReliableConnectionSendQueuePair.rs");
include!("QueuePair2.rs");
include!("RawPacketQueuePair.rs");
include!("ReliableConnectionQueuePair.rs");
include!("UnreliableConnectionQueuePair.rs");
include!("UnreliableDatagramQueuePair.rs");
