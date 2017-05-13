// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.



extern crate libc;


use ::core::mem::zeroed;
use ::core::option::Option;
use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_uint;
use ::libc::c_ulonglong;
use ::libc::c_void;

use ::libc::pthread_cond_t;
use ::libc::pthread_mutex_t;
use ::libc::timespec;


// Defined officially in linux/types.h but somewhat pointless; included here to support release v14-rc1
pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;

#[link(name = "cxgb3", kind = "static-nobundle")]
#[link(name = "cxgb4", kind = "static-nobundle")]
#[link(name = "hfi1verbs", kind = "static-nobundle")]
#[link(name = "hns", kind = "static-nobundle")]
#[link(name = "i40iw", kind = "static-nobundle")]
#[link(name = "ibcm", kind = "static-nobundle")]
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

include!("bindgen/constants.rs");
include!("bindgen/enums.rs");
include!("bindgen/functions.rs");
include!("bindgen/structs.rs");
include!("bindgen/types.rs");
include!("bindgen/unions.rs");
include!("bindgen/opaques.rs");
