// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	pub fn ucp_config_modify(config: *mut ucp_config_t, name: *const c_char, value: *const c_char) -> ucs_status_t;
	pub fn ucp_config_print(config: *const ucp_config_t, stream: *mut FILE, title: *const c_char, print_flags: ucs_config_print_flags_t);
	pub fn ucp_config_read(env_prefix: *const c_char, filename: *const c_char, config_p: *mut *mut ucp_config_t) -> ucs_status_t;
	pub fn ucp_config_release(config: *mut ucp_config_t);
}
