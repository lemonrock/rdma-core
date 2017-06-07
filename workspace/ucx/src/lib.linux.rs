// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


#[macro_use] extern crate const_cstr_fork;
#[macro_use] extern crate cfg_if;
extern crate errno;
extern crate libc;
extern crate libc_extra;
#[macro_use] extern crate quick_error;
extern crate rust_extra;
extern crate ucx_sys;


use ::errno::errno;
use ::libc::c_void;
use ::rust_extra::unlikely;
use ::std::ffi::CStr;
use ::std::mem::size_of_val;
use ::std::mem::zeroed;
use ::ucx_sys::*;


// This horrible piece of logic exists because of the incomplete state of Rust's libc wrapper; at some point sched_getcpu() for musl will be added
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


include!("ucs_cpu_set_tEx.rs");
include!("ucs_status_ptr_tEx.rs");
include!("ucs_status_tEx.rs");

