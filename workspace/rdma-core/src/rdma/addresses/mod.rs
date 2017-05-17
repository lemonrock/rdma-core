// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use ::libc::AF_INET;
use ::libc::AF_INET6;
use ::libc::c_uint;
use ::libc::sa_family_t;
use ::libc::sockaddr;
use ::libc::sockaddr_in;
use ::libc::sockaddr_in6;
use ::libc::sockaddr_storage;
use ::rdma_core_sys::*;
use ::std::mem::zeroed;
use ::std::mem::size_of;
use ::std::ptr::copy_nonoverlapping;


include!("Addressing.rs");
include!("InfinibandSocketAddress.rs");
include!("InfinibandSid.rs");
include!("IpV4SocketAddress.rs");
include!("IpV6SocketAddress.rs");
include!("Port.rs");
include!("RdmaSocketAddress.rs");
include!("SocketAddress.rs");
include!("SocketAddressCreator.rs");
