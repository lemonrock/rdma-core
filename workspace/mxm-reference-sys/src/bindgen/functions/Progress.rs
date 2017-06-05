// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_progress(context: mxm_h) -> mxm_error_t;
	pub fn mxm_progress_register(context: mxm_h, progress_cb: mxm_progress_cb_t, arg: *mut c_void) -> mxm_error_t;
	pub fn mxm_progress_unregister(context: mxm_h, progress_cb: mxm_progress_cb_t) -> mxm_error_t;
}
