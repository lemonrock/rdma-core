// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::uint16_t;
use ::libc::uint32_t;
use ::libc::uint64_t;
use ::libc::timespec;
use ::rust_extra::u31;
use ::std::mem::transmute;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ptr::null_mut;


pub mod completionQueueContexts;
pub mod model;


include!("CompletionQueuePointer.rs");
include!("HasContextPointer.rs");
include!("HasProtectionDomainPointer.rs");
include!("HasQueuePairType.rs");
include!("HasVerbsPointer.rs");
include!("ibv_async_eventEx.rs");
include!("ibv_comp_channelEx.rs");
include!("ibv_contextEx.rs");
include!("ibv_cq_exEx.rs");
include!("ibv_device_attrEx.rs");
include!("ibv_pdEx.rs");
include!("ibv_qpEx.rs");
include!("ibv_wqEx.rs");
