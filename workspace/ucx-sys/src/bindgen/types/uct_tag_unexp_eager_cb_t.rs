// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


pub type uct_tag_unexp_eager_cb_t = Option<unsafe extern "C" fn(arg: *mut c_void, data: *mut c_void, length: usize, flags: c_uint, stag: uct_tag_t, imm: u64) -> ucs_status_t>;
