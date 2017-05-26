// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use self::addresses::*;
use self::communicationIdentifierContexts::*;
use self::epoll::*;
use self::eventData::*;
use self::verbMapEntries::*;
use super::*;
use super::verbs::*;
use ::errno::errno;
use ::libc::c_int;
use ::libc::c_void;
use ::rdma_core_sys::rdma_cm_event_type::*;
use ::rdma_core_sys::rdma_port_space::*;
use ::rust_extra::likely;
use ::rust_extra::unlikely;
use ::std::collections::HashMap;
use ::std::collections::HashSet;
use ::std::marker::PhantomData;
use ::std::mem::forget;
use ::std::mem::size_of;
use ::std::mem::uninitialized;
use ::std::os::unix::io::RawFd;
use ::std::ptr::null;
use ::syscall_alt::constants::E;


pub mod addresses;
pub mod communicationIdentifierContexts;
pub mod epoll;
pub mod eventData;
pub mod verbMapEntries;


include!("ConnectionAcceptance.rs");
include!("EPollContextChoice.rs");
include!("EventChannel.rs");
include!("Listener.rs");
include!("rdma_cm_eventEx.rs");
include!("rdma_cm_idEx.rs");
include!("rdma_event_channelEx.rs");
include!("VerbMap.rs");
