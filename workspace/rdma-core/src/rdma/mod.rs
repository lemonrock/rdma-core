// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use self::communicationIdentifiers::*;
use ::libc::c_void;
use ::rdma_core_sys::*;
use ::std::cell::RefCell;
use ::std::mem::uninitialized;
use ::std::rc::Rc;


pub mod communicationIdentifiers;


include!("CommunicationEventHandler.rs");
include!("EventChannel.rs");
