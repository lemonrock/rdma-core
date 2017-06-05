// This file is part of rdma-core. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT. No part of rdma-core, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of rdma-core. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/rdma-core/master/COPYRIGHT.


extern "C"
{
	#[link_name = "stderr"] pub static stderr: *const FILE;
	#[link_name = "stdin"] pub static stdin: *const FILE;
	#[link_name = "stdout"] pub static stdout: *const FILE;
	#[link_name = "ucs_async_mode_names"] pub static mut ucs_async_mode_names: [*const c_char; 0usize];
	#[link_name = "ucs_stats_formats_names"] pub static mut ucs_stats_formats_names: [*const c_char; 0usize];
	#[link_name = "uct_alloc_method_names"] pub static mut uct_alloc_method_names: [*const c_char; 0usize];
}
