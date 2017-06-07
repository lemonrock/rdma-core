// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#![no_std]
#![cfg_attr(any(target_os="linux", target_os="android"), feature(static_nobundle))]
#![cfg_attr(any(target_os="linux", target_os="android"), feature(untagged_unions))]

#![allow(missing_copy_implementations)]
#![allow(missing_debug_implementations)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(trivial_casts)]
#![warn(unused_import_braces)]


#[cfg(any(target_os="linux", target_os="android"))] include!("bindgen/lib.rs");
