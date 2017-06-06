// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_use] extern crate cfg_if;
extern crate errno;
extern crate libc;
extern crate libc_extra;
extern crate rust_extra;
extern crate ucx_sys;


include!("panic_on_error.rs");
include!("panic_on_error_with_clean_up.rs");


use ::errno::errno;
use ::libc::c_void;
use ::libc::FILE;
use ::libc_extra::stderr;
use ::rust_extra::unlikely;
use ::std::ffi::CStr;
use ::std::ffi::CString;
use ::std::mem::size_of_val;
use ::std::mem::uninitialized;
use ::std::mem::zeroed;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::ucx_sys::*;


// This horrible piece of logic exists because of the incomplete state of Rust's libc wrapper. A good example of why development by pull request doesn't really work
cfg_if!
{
    if #[cfg(target_env = "musl")]
    {
		#[link(name = "c")]
		extern
		{
			pub fn sched_getcpu() -> ::libc::c_int;
		}
    }
    else
    {
		use ::libc::sched_getcpu;
    }
}




include!("ApplicationContext.rs");
include!("Configuration.rs");
include!("MappedMemory.rs");
include!("RemoteAccessKeyBuffer.rs");
include!("ucs_cpu_set_tEx.rs");
include!("Worker.rs");
include!("WorkerThreadMode.rs");
