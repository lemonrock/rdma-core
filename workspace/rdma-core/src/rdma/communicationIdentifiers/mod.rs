// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::c_void;
use ::rdma::addresses::Addressing;
use ::rdma::CommunicationEventHandler;
use ::rdma::EventChannel;
use ::rdma_core_sys::*;
use ::rust_extra::unlikely;
use ::std::cell::RefCell;
use ::std::marker::PhantomData;
use ::std::mem::uninitialized;
use ::std::ptr::null_mut;
use ::std::rc::Rc;


include!("AsynchronousCommunicationIdentifier.rs");
include!("ListeningSynchronousCommunicationIdentifier.rs");