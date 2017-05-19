// This file is part of dpdk. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT. No part of dpdk, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of dpdk. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/dpdk/master/COPYRIGHT.


use super::*;
use ::libc::c_int;
use ::rust_extra::unlikely;
use ::rust_extra::likely;
use ::std::ffi::CStr;
use ::std::fmt;
use ::std::fmt::Debug;
use ::std::fmt::Display;
use ::std::fmt::Formatter;
use ::std::mem::transmute;


include!("ExtendedWorkCompletion.rs");
include!("ExtendedValidWorkCompletion.rs");
include!("UnextendedWorkCompletion.rs");
include!("UnextendedValidWorkCompletion.rs");
include!("ValidWorkCompletion.rs");
include!("WorkCompletion.rs");
include!("WorkRequestError.rs");
