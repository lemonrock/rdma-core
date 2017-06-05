// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn mxm_config_free_context_opts(opts: *mut mxm_context_opts_t);
	pub fn mxm_config_free_ep_opts(opts: *mut mxm_ep_opts_t);
	pub fn mxm_config_print(stream: *mut FILE, ctx_opts: *mut mxm_context_opts_t, ep_opts: *mut mxm_ep_opts_t, flags: c_uint);
	pub fn mxm_config_read_context_opts(optsp: *mut *mut mxm_context_opts_t) -> mxm_error_t;
	pub fn mxm_config_read_ep_opts(optsp: *mut *mut mxm_ep_opts_t) -> mxm_error_t;
	pub fn mxm_config_read_opts(ctx_optsp: *mut *mut mxm_context_opts_t, ep_optsp: *mut *mut mxm_ep_opts_t, prefix: *const c_char, config_file: *const c_char, flags: c_uint) -> mxm_error_t;
}
