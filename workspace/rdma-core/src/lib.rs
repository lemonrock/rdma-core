// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![feature(associated_consts)]
#![feature(specialization)]


#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(trivial_casts)]
#![warn(unused_import_braces)]


extern crate arrayvec;
#[cfg(any(target_os="linux", target_os="android"))] #[macro_use] extern crate bitflags;
extern crate errno;
extern crate libc;
extern crate rdma_core_sys;
extern crate rust_extra;
extern crate syscall_alt;


use ::libc::c_int;
use ::rdma_core_sys::*;
use ::rust_extra::likely;
use ::rust_extra::unlikely;


include!("panic_on_errno.rs");
include!("panic_on_error.rs");
include!("panic_on_null.rs");


#[cfg(any(target_os="linux", target_os="android"))] pub mod rdma;
#[cfg(any(target_os="linux", target_os="android"))] pub mod verbs;


#[cfg(unix)] include!("FileDescriptor.rs");
include!("GlobalRoutingHeader.rs");
include!("GUID.rs");
include!("LocalIdentifier.rs");
include!("LocalIdentifierPath.rs");
include!("MemoryRegionAccess.rs");
include!("MemoryRegionWriteAccess.rs");
include!("PartitionKey.rs");
include!("PartitionKeyIndex.rs");
include!("QueuePairNumber.rs");
include!("ServiceLevel.rs");
include!("SharedReceiveQueueNumber.rs");
include!("SharedReceiveQueueSettings.rs");
include!("WorkRequestIdentifier.rs");

