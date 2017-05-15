// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(trivial_casts)]
#![warn(unused_import_braces)]


extern crate arrayvec;
extern crate errno;
extern crate libc;
extern crate rdma_core_sys;
extern crate rust_extra;
extern crate syscall_alt;


use ::completionQueues::*;
use ::completionQueues::workCompletions::*;
use ::errno::errno;
use ::libc::c_int;
use ::libc::c_void;
use ::libc::O_CREAT;
use ::libc::O_EXCL;
use ::rdma_core_sys::*;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::rust_extra::powersOfTwo::AsU32;
use ::rust_extra::powersOfTwo::PowerOfTwoThirtyTwoBit;
use ::sharedReceiveQueues::*;
use ::std::collections::HashMap;
use ::std::ffi::CStr;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ptr::null_mut;
use ::syscall_alt::constants::E;


include!("panic_on_errno.rs");
include!("panic_on_error.rs");
include!("panic_on_null.rs");


pub mod completionQueues;
pub mod sharedReceiveQueues;


include!("AddressHandle.rs");
include!("AsynchronousEvent.rs");
include!("CompletionChannel.rs");
include!("Context.rs");
include!("Device.rs");
include!("DeviceListIterator.rs");
include!("ExtendedReliableConnectionDomain.rs");
include!("FileDescriptor.rs");
include!("GlobalRoutingHeader.rs");
include!("GUID.rs");
include!("LocalIdentifier.rs");
include!("LocalIdentifierPath.rs");
include!("MemoryRegion.rs");
include!("MemoryRegionAccess.rs");
include!("MemoryRegionWriteAccess.rs");
include!("MemoryWindow.rs");
include!("PartitionKey.rs");
include!("PartitionKeyIndex.rs");
include!("Port.rs");
include!("ProtectionDomain.rs");
include!("QueuePairNumber.rs");
include!("ServiceLevel.rs");
