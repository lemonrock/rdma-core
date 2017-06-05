// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_cleanup(context: mxm_h);
	pub fn mxm_init(opts: *mut mxm_context_opts_t, context_p: *mut mxm_h) -> mxm_error_t;
	pub fn mxm_set_am_handler(context: mxm_h, hid: mxm_hid_t, cb: mxm_am_handler_t, flags: c_uint) -> mxm_error_t;
}
