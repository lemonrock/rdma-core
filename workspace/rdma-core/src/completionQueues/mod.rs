// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::Context;
use ::CompletionChannel;
use ::arrayvec::ArrayVec;
use self::workCompletions::ExtendedWorkCompletion;
use self::workCompletions::UnextendedWorkCompletion;
use ::errno::errno;
use ::rdma_core_sys::*;
use ::rust_extra::u31;
use ::rust_extra::unlikely;
use ::rust_extra::likely;
use ::std::mem::transmute;
use ::syscall_alt::constants::E;


pub mod workCompletions;


include!("CompletionQueue.rs");
include!("ExtendedCompletionQueue.rs");
include!("UnextendedCompletionQueue.rs");
include!("WithCompletionChannelCompletionQueue.rs");
include!("WithCompletionChannelExtendedCompletionQueue.rs");
include!("WithCompletionChannelUnextendedCompletionQueue.rs");
include!("WithoutCompletionChannelExtendedCompletionQueue.rs");
include!("WithoutCompletionChannelUnextendedCompletionQueue.rs");


pub const UnextendedCompletionQueuePollArraySize: usize = 32;
