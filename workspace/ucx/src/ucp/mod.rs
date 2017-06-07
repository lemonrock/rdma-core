// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::ucs_cpu_set_tEx;
use super::ucs_status_ptr_tEx;
use ::libc::c_void;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::cmp::Eq;
use ::std::cmp::Ord;
use ::std::cmp::Ordering;
use ::std::cmp::PartialEq;
use ::std::cmp::PartialOrd;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::mem::uninitialized;
use ::std::os::unix::io::RawFd;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::ucx_sys::*;

include!("panic_on_error.rs");
include!("panic_on_error_with_clean_up.rs");


pub mod failures;
pub mod genericDataTypes;


include!("ApplicationContext.rs");
include!("ApplicationContextAttributes.rs");
include!("ApplicationContextFeaturesIdeallySupported.rs");
include!("Configuration.rs");
include!("EndPoint.rs");
include!("EndPointErrorHandler.rs");
include!("MappedMemory.rs");
include!("MappedMemoryAttributes.rs");
include!("NonBlockingRequest.rs");
include!("PrintInformation.rs");
include!("QueryAttributes.rs");
include!("RemoteMemoryAccessKey.rs");
include!("RemoteMemoryAccessKeyBuffer.rs");
include!("Worker.rs");
include!("WorkerAddressHandle.rs");
include!("WorkerAttributes.rs");
include!("WorkerThreadMode.rs");
