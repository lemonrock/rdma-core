// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


extern crate libc;
extern crate libc_extra;
extern crate rust_extra;
extern crate ucx_sys;


include!("panic_on_error.rs");
include!("panic_on_error_with_clean_up.rs");


use ::libc::FILE;
use ::libc_extra::stderr;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::mem::uninitialized;
use ::std::ptr::null;
use ::ucx_sys::*;


pub mod genericDataTypes;


include!("ApplicationContext.rs");
include!("Configuration.rs");
