// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![feature(static_nobundle)]
#![feature(untagged_unions)]

#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(trivial_casts)]
#![warn(unused_import_braces)]


extern crate errno;
extern crate libc;
extern crate rdma_core_sys;
extern crate rust_extra;
extern crate syscall_alt;


use ::errno::errno;
use ::libc::c_void;
use ::rdma_core_sys::*;
use ::rust_extra::u31;
use ::rust_extra::unlikely;
use ::std::ffi::CStr;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;
use ::syscall_alt::constants::E;


include!("panic_on_errno.rs");
include!("panic_on_error.rs");
include!("panic_on_null.rs");


include!("AsynchronousEvent.rs");
include!("CompletionChannel.rs");
include!("CompletionQueue.rs");
include!("Context.rs");
include!("Device.rs");
include!("DeviceListIterator.rs");
include!("GUID.rs");
include!("Port.rs");
include!("ProtectionDomain.rs");
include!("SharedRequestQueue.rs");
include!("SharedRequestQueueSettings.rs");
