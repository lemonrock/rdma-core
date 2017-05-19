// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use self::completionQueues::*;
use self::completionQueues::workCompletions::*;
use self::sharedReceiveQueues::*;
use ::errno::errno;
use ::libc::c_void;
use ::libc::O_CREAT;
use ::libc::O_EXCL;
use ::libc::timespec;
use self::queuePairs::*;
use ::rust_extra::unlikely;
use ::rust_extra::powersOfTwo::AsU32;
use ::rust_extra::powersOfTwo::PowerOfTwoThirtyTwoBit;
use ::std::ffi::CStr;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;
use ::syscall_alt::constants::E;


pub mod completionQueues;
pub mod queuePairs;
pub mod sharedReceiveQueues;


include!("AddressHandle.rs");
include!("AsynchronousEvent.rs");
include!("Context.rs");
include!("Device.rs");
include!("DeviceListIterator.rs");
include!("ExtendedReliableConnectionDomain.rs");
include!("MemoryRegion.rs");
include!("MemoryWindow.rs");
include!("Port.rs");
include!("ProtectionDomain.rs");
