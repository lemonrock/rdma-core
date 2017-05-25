// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::arrayvec::ArrayVec;
use ::libc::c_int;
use ::libc::sigset_t;
use ::rdma_core_sys::*;
use ::rust_extra::likely;
use ::rust_extra::u31;
use ::rust_extra::unlikely;
use ::std::mem::transmute;
use ::std::os::unix::io::RawFd;
use ::std::ptr::null;
use ::std::ptr::null_mut;
use ::syscall_alt::PosixErrorNumber;
use ::syscall_alt::SyscallArgument;
use ::syscall_alt::constants::NegativeE;
use ::syscall_alt::constants::linux_like::_NSIG;
use ::syscall_alt::syscalls::Syscall;


include!("EPollContext.rs");
include!("EPollEvents.rs");
include!("EPollFileDescriptor.rs");
include!("MaximumEvents.rs");
include!("RawEPollFileDescriptor.rs");
include!("RawFdEx.rs");
