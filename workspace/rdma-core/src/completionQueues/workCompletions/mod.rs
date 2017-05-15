// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::LocalIdentifier;
use ::LocalIdentifierPath;
use ::PartitionKeyIndex;
use ::QueuePairNumber;
use ::ServiceLevel;
use ::completionQueues::ExtendedCompletionQueue;
use ::libc::c_int;
use ::rdma_core_sys::*;
use ::rust_extra::unlikely;
use ::rust_extra::likely;
use ::std::mem::transmute;


include!("ExtendedWorkCompletion.rs");
include!("ExtendedValidWorkCompletion.rs");
include!("UnextendedWorkCompletion.rs");
include!("UnextendedValidWorkCompletion.rs");
include!("ValidWorkCompletion.rs");
include!("WorkCompletion.rs");
include!("WorkRequestError.rs");
include!("WorkRequestIdentifier.rs");
