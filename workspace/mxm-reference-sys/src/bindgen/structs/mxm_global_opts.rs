// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


#[repr(C)]
#[derive(Debug, Copy)]
pub struct mxm_global_opts
{
	pub log_level: mxm_log_level_t,
	pub log_file: *mut c_char,
	pub log_buffer_size: usize,
	pub log_data_size: usize,
	pub handle_errors: mxm_handle_error_t,
	pub error_signals: mxm_global_opts__bindgen_ty_1,
	pub gdb_command: *mut c_char,
	pub debug_signo: c_uint,
	pub instrument_file: *mut c_char,
	pub instrument_max_size: usize,
	pub async_interval: f64,
	pub stats_dest: *mut c_char,
	pub stats_trigger: *mut c_char,
	pub tuning_path: *mut c_char,
	pub perf_stall_loops: usize,
	pub async_signo: c_uint,
	pub memtrack_dest: *mut c_char,
}
