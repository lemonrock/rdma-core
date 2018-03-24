// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.



extern crate libc;


use ::std::clone::Clone;
use ::std::default::Default;
use ::std::fmt::Debug;
use ::std::fmt::Formatter;
use ::std::fmt::Result;
use ::std::marker::Copy;
use ::std::marker::PhantomData;
use ::std::mem::transmute;
use ::std::mem::zeroed;
use ::std::option::Option;
use ::std::ops::BitOr;
use ::std::ops::BitOrAssign;
use ::std::ops::BitAnd;
use ::std::ops::BitAndAssign;
use ::std::hash::Hash;
use ::std::hash::Hasher;
use ::std::cmp::PartialEq;
use ::std::cmp::Eq;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_ushort;
use ::libc::c_void;

use ::libc::pthread_cond_t;
use ::libc::pthread_mutex_t;
use ::libc::sockaddr;
use ::libc::sockaddr_in;
use ::libc::sockaddr_in6;
use ::libc::sockaddr_storage;
use ::libc::socklen_t;
use ::libc::timespec;


// Defined officially in linux/types.h; created here to make it easier to generate this code
pub type __u8 = u8;
pub type __u64 = u64;
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;

#[link(name = "cxgb3", kind = "static-nobundle")]
#[link(name = "cxgb4", kind = "static-nobundle")]
#[link(name = "hfi1verbs", kind = "static-nobundle")]
#[link(name = "hns", kind = "static-nobundle")]
#[link(name = "i40iw", kind = "static-nobundle")]
#[link(name = "ibumad", kind = "static-nobundle")]
#[link(name = "ibverbs", kind = "static-nobundle")]
#[link(name = "ipathverbs", kind = "static-nobundle")]
#[link(name = "mlx4", kind = "static-nobundle")]
#[link(name = "mlx5", kind = "static-nobundle")]
#[link(name = "mthca", kind = "static-nobundle")]
#[link(name = "nes", kind = "static-nobundle")]
#[link(name = "ocrdma", kind = "static-nobundle")]
#[link(name = "qedr", kind = "static-nobundle")]
#[link(name = "rdmacm", kind = "static-nobundle")]
#[link(name = "rxe", kind = "static-nobundle")]
#[link(name = "vmw_pvrdma", kind = "static-nobundle")]
extern "C"
{
}

include!("constants.rs");
include!("functions.rs");
include!("structs.rs");
include!("types.rs");
include!("unions.rs");
