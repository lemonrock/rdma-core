// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.



extern crate libc;


use ::libc::c_char;
use ::libc::c_int;
use ::libc::c_long;
use ::libc::c_longlong;
use ::libc::c_short;
use ::libc::c_uint;
use ::libc::c_void;


include!("bindgen/constants.rs");
include!("bindgen/functions.rs");
include!("bindgen/structs.rs");
